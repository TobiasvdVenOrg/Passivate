mod helpers;

#[macro_use]
extern crate assert_matches;

use std::io::Error as IoError;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use galvanic_assert::assert_that;
use galvanic_assert::matchers::collection::contains_in_order;
use itertools::Itertools;
use mockall::Sequence;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::test_name;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::hyp_run_request::{HypRunOptions, HypRunRequest, HypRunRequestKind};
use passivate_model_bridge::hyp_session_bridge::{self, CancelRunBridge, CompleteRunBridge, MockHypSessionBridge};
use passivate_model_bridge::hyp_session_event::{ConsoleOutput, HypSessionEvent};
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_core::hyp_session::HypSession;
use passivate_run_rust::hyp_run_error::HypRunError;
use passivate_run_rust::hyp_run_handler::hyp_run_thread;
use passivate_run_rust::hyp_runner::MockRunHyps;
use passivate_run_rust::model::{RustBridge, RustOutput};
use passivate_run_rust::nextest_error::NextestError;
use passivate_testing::test_data_setup::TestDataSetup;
use passivate_testing::test_snapshot_path::TestSnapshotPath;

use crate::helpers::HandleHypRunTrigger;

#[test]
pub fn hyp_run_thread_cancels_run_upon_new_request()
{
    let (hyp_run_trigger_tx, hyp_run_trigger_rx) = crossbeam_channel::unbounded();
    let mut hyp_session_bridge = MockHypSessionBridge::new();

    let mut cancel_then_complete = Sequence::new();

    hyp_session_bridge.expect_start_run();

    hyp_session_bridge
        .expect_cancel_run()
        .times(1)
        .in_sequence(&mut cancel_then_complete);

    hyp_session_bridge
        .expect_complete_run()
        .times(1)
        .in_sequence(&mut cancel_then_complete);

    let mut runner = MockRunHyps::new();
    runner
        .expect_run_hyps()
        .returning(|_, _: &mut MockHypSessionBridge<RustBridge>, _| {
            thread::sleep(Duration::from_secs(1)); // sleep to give time for cancellation to come in
            Ok(())
        });

    thread::scope(|scope| {
        _ = hyp_run_thread(scope, hyp_run_trigger_rx, hyp_session_bridge, runner);

        hyp_run_trigger_tx
            .send(HypRunRequest {
                kind: HypRunRequestKind::All,
                options: HypRunOptions::default()
            })
            .unwrap();

        hyp_run_trigger_tx
            .send(HypRunRequest {
                kind: HypRunRequestKind::All,
                options: HypRunOptions::default()
            })
            .unwrap();

        drop(hyp_run_trigger_tx);

        // t.join().unwrap();

        // eprintln!("oi");
    });
}

#[test]
pub fn running_single_hyp_leaves_session_in_passed_state()
{
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build().clean_output();

    let hyp_to_run = HypId::new("simple_project", "simple_project", "add_8_and_8_is_16");

    HandleHypRunTrigger::new()
        .with_runner_from_setup(&setup)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::single(hyp_to_run, HypRunOptions::default()));

    let session = HypSession::from_events(session_rx.try_iter());

    assert_matches!(session.state(), HypState::Passed);
}

#[test]
pub fn single_hyp_run_only_runs_one_exact_hyp()
{
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build().clean_output();

    let hyp_to_run = HypId::new("simple_project", "simple_project", "add_2_and_2_is_4");

    HandleHypRunTrigger::new()
        .with_runner_from_setup(&setup)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::single(hyp_to_run, HypRunOptions::default()));

    let session = HypSession::from_events(session_rx.try_iter());
    let mut iter = session.hyps().iter();

    assert_matches!(iter.next(), Some(_hyp_to_run));
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

    let mut handle_hyp_run = HandleHypRunTrigger::new().with_runner_from_setup(&setup);

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

    let mut handle_hyp_run = HandleHypRunTrigger::new().with_runner_from_setup(&setup);

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

/// This test assumes RUST_BACKTRACE=0 (the extra lines in the output are otherwise unaccounted for)
#[test]
pub fn failing_tests_output_is_captured_in_state() -> Result<(), IoError>
{
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project_failing_tests")
        .build()
        .clean_output();

    HandleHypRunTrigger::new()
        .with_runner_from_setup(&setup)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::all(HypRunOptions::default()));

    let failed_test = HypId::new("sample_project", "multiply_tests", "multiply_2_and_2_is_4");

    let session = HypSession::from_events(session_rx.try_iter());

    let failed_test = session.hyps().get(failed_test.chain()).unwrap();

    let expected = [
        RustOutput::Console(ConsoleOutput::new_stderr("assertion `left == right` failed")),
        RustOutput::Console(ConsoleOutput::new_stderr("  left: 5")),
        RustOutput::Console(ConsoleOutput::new_stderr(" right: 4")),
        RustOutput::Console(ConsoleOutput::new_stderr(
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        ))
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
    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    let setup = TestDataSetup::builder(test_name!(), "simple_project_failing_tests")
        .build()
        .clean_output();

    let mut handle_hyp_run = HandleHypRunTrigger::new()
        .with_runner_from_setup(&setup)
        .with_hyp_session_bridge(session_tx);

    // Run tests twice
    handle_hyp_run.call(HypRunRequest::all(HypRunOptions::default()));
    handle_hyp_run.call(HypRunRequest::all(HypRunOptions::default()));

    let failed_hyp = HypId::new("sample_project", "multiply_tests", "multiply_2_and_2_is_4");

    let session = HypSession::from_events(session_rx.try_iter());

    let failed_test = session.hyps().get(&failed_hyp).unwrap();

    let expected = [
        RustOutput::Console(ConsoleOutput::new_stderr("assertion `left == right` failed")),
        RustOutput::Console(ConsoleOutput::new_stderr("  left: 5")),
        RustOutput::Console(ConsoleOutput::new_stderr(" right: 4")),
        RustOutput::Console(ConsoleOutput::new_stderr(
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"
        ))
    ];

    assert_that!(
        // Skip first 2 lines to avoid a thread ID that is not deterministic
        &failed_test.iter_output().skip(2).collect::<Vec<_>>(),
        contains_in_order(expected.iter())
    );

    Ok(())
}

#[test]
pub fn when_hyp_run_fails_error_is_reported()
{
    let mut run_hyps = MockRunHyps::new();
    run_hyps
        .expect_run_hyps::<crossbeam_channel::Sender<HypSessionEvent<RustBridge>>>()
        .returning(|_, _, _| Err(HypRunError::Nextest(Arc::new(NextestError::UnknownFiltersetParse))));

    let (session_tx, session_rx) = crossbeam_channel::unbounded();

    HandleHypRunTrigger::new()
        .with_runner(run_hyps)
        .with_hyp_session_bridge(session_tx)
        .call(HypRunRequest::all(HypRunOptions::default()));

    let session = HypSession::from_events(session_rx.try_iter());

    assert_matches!(session.activity(), Ok(HypState::Failed));
}
