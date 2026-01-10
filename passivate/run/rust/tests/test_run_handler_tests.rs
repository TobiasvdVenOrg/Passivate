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
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::test_name;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge_hyp::BridgeHyp;
use passivate_model_bridge::hyp_run_request::{HypRunOptions, HypRunRequest, HypRunRequestKind};
use passivate_model_bridge::hyp_session_event::ConsoleOutput;
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_rust::{RustHyp, RustOutput};
use passivate_run_core::hyp_run_errors::TestRunError;
use passivate_run_rust::hyp_runner::HypRunner;
use passivate_testing::test_data_setup::TestDataSetup;
use passivate_testing::test_snapshot_path::TestSnapshotPath;

use crate::helpers::HandleHypRunTrigger;

#[test]
pub fn runing_single_hyp_leaves_session_in_passed_state()
{
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build().clean_output();

    let hyp_to_run = HypId::new("simple_project", "simple_project", "add_8_and_8_is_16");

    HandleHypRunTrigger::new(&setup)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::single(hyp_to_run, HypRunOptions::default()));

    let session = HypSession::from_events(session_rx);

    assert!(session.state() == HypState::Passed);
}

#[test]
pub fn single_hyp_run_only_runs_one_exact_hyp()
{
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build().clean_output();

    let hyp_to_run = HypId::new("simple_project", "simple_project", "add_2_and_2_is_4");

    HandleHypRunTrigger::new(&setup)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::single(hyp_to_run, HypRunOptions::default()));

    let session = HypSession::from_events(session_rx);
    let mut iter = session.hyps().iter();

    assert_matches!(iter.next(), Some(hyp_to_run));
    assert_matches!(iter.next(), None);
}

#[test]
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

    let mut handle_hyp_run = HandleHypRunTrigger::new(&setup);

    // Run all tests first to generate a new snapshot
    handle_hyp_run.call(HypRunRequest::all(HypRunOptions::default()));

    let snapshot_hyp_id = HypId::new("simple_project", "snapshot_tests", "snapshot_test");

    handle_hyp_run.call(HypRunRequest::single(
        snapshot_hyp_id,
        HypRunOptions {
            update_snapshots: true,
            ..HypRunOptions::default()
        }
    ));

    assert!(fs::exists(expected_approved_snapshot)?);

    Ok(())
}

#[test]
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

    let mut handle_hyp_run = HandleHypRunTrigger::new(&setup);

    // Run all tests first to generate a new snapshot
    handle_hyp_run.call(HypRunRequest::all(HypRunOptions::default()));

    let snapshot_hyp_id = HypId::new("simple_project", "snapshot_tests", "snapshot_test");

    handle_hyp_run.call(HypRunRequest::single(
        snapshot_hyp_id,
        HypRunOptions {
            update_snapshots: true,
            ..HypRunOptions::default()
        }
    ));

    // Run all tests again, which should not approve snapshots
    handle_hyp_run.call(HypRunRequest::all(HypRunOptions::default()));

    assert!(fs::exists(expected_approved_snapshot)?);
    assert!(fs::exists(expected_unapproved_snapshot)?);

    Ok(())
}

#[test]
pub fn failing_tests_output_is_captured_in_state() -> Result<(), IoError>
{
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project_failing_tests")
        .build()
        .clean_output();

    HandleHypRunTrigger::new(&setup)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::all(HypRunOptions::default()));

    let failed_test = HypId::new("simple_project", "multiply_tests", "multiply_2_and_2_is_4");

    let session = HypSession::from_events(session_rx);

    let failed_test = session.hyps().entry(failed_test.chain()).unwrap();

    let expected = vec![
        RustOutput::Console(ConsoleOutput::new_stderr("assertion `left == right` failed")),
        RustOutput::Console(ConsoleOutput::new_stderr("  left: 5")),
        RustOutput::Console(ConsoleOutput::new_stderr(" right: 4")),
        RustOutput::Console(ConsoleOutput::new_stderr(
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        )),
    ];
    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.iter_output().skip(2).collect::<Vec<_>>(),
        contains_in_order(expected.iter())
    );

    Ok(())
}

#[test]
pub fn failing_tests_output_persists_on_repeat_runs() -> Result<(), IoError>
{
    use passivate_run_core::session_event_tx::SessionEventTx;

    let (hyp_run_tx, hyp_run_rx) = SessionEventTx::new();

    let setup = TestDataSetup::builder(test_name!(), "simple_project_failing_tests")
        .build()
        .clean_output();

    let mut handler = helpers::test_hyp_run_handler(&setup).hyp_run_tx(hyp_run_tx).call();

    // Run tests twice
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());
    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let failed_hyp = HypId::new("simple_project", "multiply_tests", "multiply_2_and_2_is_4");

    let session = HypSession::from_events(hyp_run_rx);

    let failed_test = session.hyps().entry(&failed_hyp).unwrap();

    let expected = vec![
        RustOutput::Console(ConsoleOutput::new_stderr("assertion `left == right` failed")),
        RustOutput::Console(ConsoleOutput::new_stderr("  left: 5")),
        RustOutput::Console(ConsoleOutput::new_stderr(" right: 4")),
        RustOutput::Console(ConsoleOutput::new_stderr(
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        )),
    ];

    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.iter_output().skip(2).collect::<Vec<_>>(),
        contains_in_order(expected.iter())
    );

    Ok(())
}

#[test]
pub fn when_test_run_fails_error_is_reported()
{
    let mut test_runner = HypRunner::faux();
    test_runner._when_run_hyps().then(|_| Err(TestRunError::Cancelled(Cancelled)));

    let (hyp_run_tx, hyp_run_rx) = SessionEventTx::new();

    let mut handler = HypRunHandler::builder()
        .runner(test_runner)
        .coverage(Box::new(compute_coverage::stub()))
        .hyp_run_tx(hyp_run_tx)
        .coverage_tx(Tx::stub())
        .configuration(ConfigurationManager::default_config(Tx::stub()))
        .build();

    handler.handle(HypRunTrigger::DefaultRun, Cancellation::default());

    let session = HypSession::from_events(hyp_run_rx);

    assert_matches!(session.activity(), Err(_));
}
