use std::fs;
use std::io::Error as IoError;

use galvanic_assert::matchers::collection::contains_in_order;
use galvanic_assert::matchers::*;
use galvanic_assert::{assert_that, is_variant};
use itertools::Itertools;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::compute_coverage;
use passivate_delegation::{Cancellation, Cancelled, Tx};
use passivate_hyp_execution::test_helpers::test_run_setup::TestRunSetup;
use passivate_hyp_execution::test_helpers::test_snapshot_path::TestSnapshotPath;
use passivate_hyp_execution::test_run_errors::TestRunError;
use passivate_hyp_execution::test_run_handler::TestRunHandler;
use passivate_hyp_execution::hyp_runner::HypRunner;
use passivate_hyp_model::change_event::ChangeEvent;
use passivate_hyp_model::single_hyp_status::SingleHypStatus;
use passivate_hyp_model::test_run::{FailedTestRun, TestRun, TestRunState};
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::test_name;
use pretty_assertions::assert_eq;

#[test]
#[cfg(target_os = "windows")]
pub fn handle_single_test_run()
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .hyp_run_tx(hyp_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    let hyp_to_run = HypId::new("simple_project", "add_8_and_8_is_16").unwrap();

    handler.handle(
        ChangeEvent::SingleHyp {
            id: hyp_to_run,
            update_snapshots: false
        },
        Cancellation::default()
    );

    let test_run = TestRun::from_events(hyp_run_rx);

    assert_that!(&test_run.state, is_variant!(TestRunState::Idle));
    assert!(
        test_run
            .tests
            .into_iter()
            .all(|test| { test.status == SingleHypStatus::Passed })
    );
}

#[test]
pub fn single_hyp_run_only_runs_one_exact_hyp()
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .hyp_run_tx(hyp_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    let hyp_to_run = HypId::new("add_tests", "add_2_and_2_is_4").unwrap();

    handler.handle(
        ChangeEvent::SingleHyp {
            id: hyp_to_run.clone(),
            update_snapshots: false
        },
        Cancellation::default()
    );

    let test_run = TestRun::from_events(hyp_run_rx);

    let single_hyp = test_run.tests.into_iter().exactly_one().expect("expected exactly one test");
    assert_that!(&single_hyp.id, eq(hyp_to_run));
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_is_pinned_only_that_test_is_run_when_changes_are_handled()
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .hyp_run_tx(hyp_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let test_run = TestRun::from_events(hyp_run_rx.drain());

    let pinned_hyp = test_run.tests.into_iter().next().unwrap();

    handler.handle(
        ChangeEvent::PinHyp {
            id: pinned_hyp.id.clone()
        },
        Cancellation::default()
    );
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let pinned_run = TestRun::from_events(hyp_run_rx.drain());

    // Assert that all tests are unknown, except the pinned test, which is passing
    assert!(pinned_run.tests.into_iter().all(|test| {
        (test.id == pinned_hyp.id && test.status == SingleHypStatus::Passed) || test.status == SingleHypStatus::Unknown
    }));
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_is_unpinned_all_tests_are_run_when_changes_are_handled()
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();
    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .hyp_run_tx(hyp_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let all_hyps = TestRun::from_events(hyp_run_rx.drain());

    let pinned_hyp = all_hyps.tests.into_iter().next().unwrap().to_owned();

    handler.handle(ChangeEvent::PinHyp { id: pinned_hyp.id }, Cancellation::default());
    handler.handle(ChangeEvent::ClearPinnedHyps, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let test_run = TestRun::from_events(hyp_run_rx.drain());

    // Assert that all tests are unknown, except the pinned test, which is passing
    assert!(
        test_run
            .tests
            .into_iter()
            .all(|test| { test.status == SingleHypStatus::Passed })
    );
}

#[test]
#[cfg(target_os = "windows")]
pub fn update_snapshots_replaces_snapshot_with_approved() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "project_snapshot_tests")
        .override_snapshot_directories(vec!(TestSnapshotPath::relative_to_output("snapshots")))
        .build()
        .clean_snapshots();

    let snapshots_dir = setup.get_snapshot_directories().into_iter().exactly_one().unwrap();

    // The sample project uses this envvar to determine where to output snapshots
    // The purpose is so that multiple tests can re-use the same sample project
    // but have separate snapshot directories that don't interfere with each other
    unsafe { std::env::set_var("PASSIVATE_SNAPSHOT_DIR", &snapshots_dir); }

    let expected_approved_snapshot = snapshots_dir.join("example_snapshot.png");
    let mut handler = setup.build_test_run_handler();

    // Run all tests first to generate a new snapshot
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let snapshot_hyp_id = HypId::new("snapshot_tests", "snapshot_test").unwrap();

    handler.handle(
        ChangeEvent::SingleHyp {
            id: snapshot_hyp_id,
            update_snapshots: true
        },
        Cancellation::default()
    );

    assert_that!(fs::exists(expected_approved_snapshot)?);

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn updating_a_snapshot_only_updates_one_exact_snapshot() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "project_snapshot_tests")
        .override_snapshot_directories(vec![TestSnapshotPath::relative_to_output("snapshots")])
        .build()
        .clean_snapshots();

    let snapshots_dir = setup.get_snapshot_directories().into_iter().exactly_one().unwrap();

    // The sample project uses this envvar to determine where to output snapshots
    // The purpose is so that multiple tests can re-use the same sample project
    // but have separate snapshot directories that don't interfere with each other
    unsafe { std::env::set_var("PASSIVATE_SNAPSHOT_DIR", &snapshots_dir); }

    let expected_approved_snapshot = snapshots_dir.join("example_snapshot.png");
    let expected_unapproved_snapshot = snapshots_dir.join("different_example_snapshot.new.png");

    let mut handler = setup.build_test_run_handler();

    // Run all tests first to generate a new snapshot
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let snapshot_hyp_id = HypId::new("snapshot_tests", "snapshot_test").unwrap();

    handler.handle(
        ChangeEvent::SingleHyp {
            id: snapshot_hyp_id,
            update_snapshots: true
        },
        Cancellation::default()
    );

    // Run all tests again, which should no approve snapshots
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    assert_that!(fs::exists(expected_approved_snapshot)?);
    assert_that!(fs::exists(expected_unapproved_snapshot)?);

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn failing_tests_output_is_captured_in_state() -> Result<(), IoError>
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project_failing_tests")
        .hyp_run_tx(hyp_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first to generate a new snapshot
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let failed_test = HypId::new("multiply_tests", "multiply_2_and_2_is_4").unwrap();

    let hyp_run = TestRun::from_events(hyp_run_rx);

    let failed_test = hyp_run.tests.find(&failed_test).unwrap();

    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.output.clone().into_iter().skip(2).collect::<Vec<_>>(),
        contains_in_order(vec![
            "assertion `left == right` failed".to_string(),
            "  left: 5".to_string(),
            " right: 4".to_string(),
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace".to_string()
        ])
    );

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn failing_tests_output_persists_on_repeat_runs() -> Result<(), IoError>
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project_failing_tests")
        .hyp_run_tx(hyp_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run tests twice
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let failed_hyp = HypId::new("multiply_tests", "multiply_2_and_2_is_4").unwrap();

    let hyp_run = TestRun::from_events(hyp_run_rx);

    let failed_test = hyp_run.tests.find(&failed_hyp).unwrap();

    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.output.clone().into_iter().skip(2).collect::<Vec<_>>(),
        contains_in_order(vec![
            "assertion `left == right` failed".to_string(),
            "  left: 5".to_string(),
            " right: 4".to_string(),
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace".to_string()
        ])
    );

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_run_fails_error_is_reported()
{
    let mut test_runner = HypRunner::faux();
    test_runner._when_run_hyps().then(|_| Err(TestRunError::Cancelled(Cancelled)));

    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let mut handler = TestRunHandler::builder()
        .runner(test_runner)
        .coverage(Box::new(compute_coverage::stub()))
        .hyp_run_tx(hyp_run_tx)
        .coverage_status_sender(Tx::stub())
        .configuration(ConfigurationManager::default_config(Tx::stub()))
        .build();

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let state = TestRun::from_events(hyp_run_rx).state;

    assert_eq!(
        state,
        TestRunState::Failed(FailedTestRun {
            inner_error_display: "test run cancelled".to_string()
        })
    );
}
