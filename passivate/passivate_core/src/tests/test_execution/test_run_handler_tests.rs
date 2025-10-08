use std::fs;
use std::io::Error as IoError;

use galvanic_assert::matchers::collection::contains_in_order;
use galvanic_assert::{assert_that, is_variant};
use passivate_delegation::{Cancellation, Cancelled, Tx};
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::test_name;
use pretty_assertions::assert_eq;

use crate::change_events::ChangeEvent;
use crate::configuration::ConfigurationManager;
use crate::coverage::compute_coverage;
use crate::test_execution::{TestRunError, TestRunHandler, TestRunner};
use crate::test_helpers::test_run_setup::TestRunSetup;
use crate::test_run_model::{FailedTestRun, SingleTestStatus, TestRunState};

#[test]
#[cfg(target_os = "windows")]
pub fn handle_single_test_run()
{
    let (test_run_tx, test_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    let hyp_to_run_again = HypId::new("simple_project", "add_8_and_8_is_16").unwrap();

    handler.handle(
        ChangeEvent::SingleHyp {
            id: hyp_to_run_again,
            update_snapshots: false
        },
        Cancellation::default()
    );

    let final_state = test_run_rx.last().unwrap();

    assert_that!(&final_state.state, is_variant!(TestRunState::Idle));
    assert!(final_state.tests.iter().all(|test| { test.status == SingleTestStatus::Passed }));
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_is_pinned_only_that_test_is_run_when_changes_are_handled()
{
    let (test_run_tx, test_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let all_hyps = test_run_rx.last().unwrap();

    let pinned_hyp = all_hyps.tests.iter().next().unwrap().to_owned();

    handler.handle(ChangeEvent::PinHyp { id: pinned_hyp.id() }, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let pinned_run = test_run_rx.last().unwrap();
    // Assert that all tests are unknown, except the pinned test, which is passing
    assert!(
        pinned_run
            .tests
            .iter()
            .all(|test| { (test.id() == pinned_hyp.id() && test.status == SingleTestStatus::Passed) || test.status == SingleTestStatus::Unknown })
    );
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_is_unpinned_all_tests_are_run_when_changes_are_handled()
{
    let (test_run_tx, test_run_rx) = Tx::new();
    let mut handler = TestRunSetup::builder(test_name!(), "simple_project")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let all_hyps = test_run_rx.last().unwrap();

    let pinned_hyp = all_hyps.tests.iter().next().unwrap().to_owned();

    handler.handle(ChangeEvent::PinHyp { id: pinned_hyp.id() }, Cancellation::default());
    handler.handle(ChangeEvent::ClearPinnedHyps, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let test_run = test_run_rx.last().unwrap();
    // Assert that all tests are unknown, except the pinned test, which is passing
    assert!(test_run.tests.iter().all(|test| { test.status == SingleTestStatus::Passed }));
}

#[test]
#[cfg(target_os = "windows")]
pub fn update_snapshots_replaces_snapshot_with_approved() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(test_name!(), "project_snapshot_tests").build().clean_snapshots();

    let expected_approved_snapshot = setup.get_snapshots_path().join("example_snapshot.png");
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
pub fn failing_tests_output_is_captured_in_state() -> Result<(), IoError>
{
    let (test_run_tx, test_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project_failing_tests")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first to generate a new snapshot
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let failed_test = HypId::new("multiply_tests", "multiply_2_and_2_is_4").unwrap();

    let state = test_run_rx.last().unwrap();

    let failed_test = state.tests.find(&failed_test).unwrap();

    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.output.into_iter().skip(2).collect::<Vec<_>>(),
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
    let (test_run_tx, test_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(test_name!(), "simple_project_failing_tests")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run tests twice
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let failed_hyp = HypId::new("multiply_tests", "multiply_2_and_2_is_4").unwrap();

    let state = test_run_rx.last().unwrap();

    let failed_test = state.tests.find(&failed_hyp).unwrap();

    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.output.into_iter().skip(2).collect::<Vec<_>>(),
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
    let mut test_runner = TestRunner::faux();
    test_runner._when_run_hyps().then(|_| Err(TestRunError::Cancelled(Cancelled)));

    let (test_run_tx, test_run_rx) = Tx::new();

    let mut handler = TestRunHandler::builder()
        .runner(test_runner)
        .coverage(Box::new(compute_coverage::stub()))
        .tests_status_sender(test_run_tx)
        .coverage_status_sender(Tx::stub())
        .log(Tx::stub())
        .configuration(ConfigurationManager::default_config(Tx::stub()))
        .build();

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let last = test_run_rx.last().unwrap().state;

    assert_eq!(
        last,
        TestRunState::Failed(FailedTestRun {
            inner_error_display: "test run cancelled".to_string()
        })
    );
}
