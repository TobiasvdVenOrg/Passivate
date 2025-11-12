mod helpers;

#[macro_use]
extern crate assert_matches;

use std::fs;
use std::io::Error as IoError;

use galvanic_assert::assert_that;
use galvanic_assert::matchers::collection::contains_in_order;
use itertools::Itertools;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::compute_coverage;
use passivate_delegation::{Cancellation, Cancelled, Tx};
use passivate_hyp_execution::hyp_run_errors::TestRunError;
use passivate_hyp_execution::hyp_run_handler::HypRunHandler;
use passivate_hyp_execution::hyp_runner::HypRunner;
use passivate_hyp_model::hyp_iter_ext::HypIterator;
use passivate_hyp_model::hyp_run_trigger::HypRunTrigger;
use passivate_hyp_model::hyp_session::HypSession;
use passivate_hyp_model::hyp_state::HypState;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::test_name;
use passivate_testing::test_data_setup::TestDataSetup;
use passivate_testing::test_snapshot_path::TestSnapshotPath;

#[test]
#[cfg(target_os = "windows")]
pub fn handle_single_test_run()
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build().clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup).hyp_run_tx(hyp_run_tx).call();

    let hyp_to_run = HypId::new("simple_project", "simple_project", "add_8_and_8_is_16");

    handler.handle(
        HypRunTrigger::Hyp {
            id: hyp_to_run,
            update_snapshots: false
        },
        Cancellation::default()
    );

    let session = HypSession::from_events(hyp_run_rx);

    assert!(session.all_hyps().current_state() == HypState::Passed);
}

#[test]
pub fn single_hyp_run_only_runs_one_exact_hyp()
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build().clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup).hyp_run_tx(hyp_run_tx).call();

    let hyp_to_run = HypId::new("simple_project", "add_tests", "add_2_and_2_is_4");

    handler.handle(
        HypRunTrigger::Hyp {
            id: hyp_to_run.clone(),
            update_snapshots: false
        },
        Cancellation::default()
    );

    let session = HypSession::from_events(hyp_run_rx);

    assert_matches!(session.all_hyps().exactly_one(), Ok(hyp_to_run));
}

#[test]
#[cfg(target_os = "windows")]
pub fn update_snapshots_replaces_snapshot_with_approved() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "project_snapshot_tests")
        .override_snapshot_directories(vec![TestSnapshotPath::relative_to_output("snapshots")])
        .build()
        .clean_snapshots();

    let snapshots_dir = setup.snapshot_directories().into_iter().exactly_one().unwrap();

    // The sample project uses this envvar to determine where to output snapshots
    // The purpose is so that multiple tests can re-use the same sample project
    // but have separate snapshot directories that don't interfere with each other
    unsafe {
        std::env::set_var("PASSIVATE_SNAPSHOT_DIR", &snapshots_dir);
    }

    let expected_approved_snapshot = snapshots_dir.join("example_snapshot.png");
    let mut handler = helpers::test_hyp_run_handler(&setup).call();

    // Run all tests first to generate a new snapshot
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let snapshot_hyp_id = HypId::new("simple_project", "snapshot_tests", "snapshot_test");

    handler.handle(
        HypRunTrigger::Hyp {
            id: snapshot_hyp_id,
            update_snapshots: true
        },
        Cancellation::default()
    );

    assert!(fs::exists(expected_approved_snapshot)?);

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn updating_a_snapshot_only_updates_one_exact_snapshot() -> Result<(), IoError>
{
    let setup = TestDataSetup::builder(test_name!(), "project_snapshot_tests")
        .override_snapshot_directories(vec![TestSnapshotPath::relative_to_output("snapshots")])
        .build()
        .clean_snapshots();

    let snapshots_dir = setup.snapshot_directories().into_iter().exactly_one().unwrap();

    // The sample project uses this envvar to determine where to output snapshots
    // The purpose is so that multiple tests can re-use the same sample project
    // but have separate snapshot directories that don't interfere with each other
    unsafe {
        std::env::set_var("PASSIVATE_SNAPSHOT_DIR", &snapshots_dir);
    }

    let expected_approved_snapshot = snapshots_dir.join("example_snapshot.png");
    let expected_unapproved_snapshot = snapshots_dir.join("different_example_snapshot.new.png");

    let mut handler = helpers::test_hyp_run_handler(&setup).call();

    // Run all tests first to generate a new snapshot
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let snapshot_hyp_id = HypId::new("simple_project", "snapshot_tests", "snapshot_test");

    handler.handle(
        HypRunTrigger::Hyp {
            id: snapshot_hyp_id,
            update_snapshots: true
        },
        Cancellation::default()
    );

    // Run all tests again, which should no approve snapshots
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    assert!(fs::exists(expected_approved_snapshot)?);
    assert!(fs::exists(expected_unapproved_snapshot)?);

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn failing_tests_output_is_captured_in_state() -> Result<(), IoError>
{
    let (hyp_run_tx, hyp_run_rx) = Tx::new();

    let setup = TestDataSetup::builder(test_name!(), "simple_project_failing_tests")
        .build()
        .clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup).hyp_run_tx(hyp_run_tx).call();

    // Run all tests first to generate a new snapshot
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let failed_test = HypId::new("simple_project", "multiply_tests", "multiply_2_and_2_is_4");

    let session = HypSession::from_events(hyp_run_rx);

    let failed_test = session.all_hyps().by_id(&failed_test).unwrap();

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

    let setup = TestDataSetup::builder(test_name!(), "simple_project_failing_tests")
        .build()
        .clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup).hyp_run_tx(hyp_run_tx).call();

    // Run tests twice
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let failed_hyp = HypId::new("simple_project", "multiply_tests", "multiply_2_and_2_is_4");

    let session = HypSession::from_events(hyp_run_rx);

    let failed_test = session.all_hyps().by_id(&failed_hyp).unwrap();

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

    let mut handler = HypRunHandler::builder()
        .runner(test_runner)
        .coverage(Box::new(compute_coverage::stub()))
        .hyp_run_tx(hyp_run_tx)
        .coverage_tx(Tx::stub())
        .configuration(ConfigurationManager::default_config(Tx::stub()))
        .build();

    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let session = HypSession::from_events(hyp_run_rx);

    assert_matches!(session.state(), Err(_));
}
