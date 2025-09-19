use std::fs;
use std::io::Error as IoError;

use galvanic_assert::matchers::collection::contains_in_order;
use galvanic_assert::{assert_that, is_variant};
use passivate_delegation::{Cancellation, Cancelled, Tx};
use pretty_assertions::assert_eq;
use stdext::function_name;

use crate::change_events::ChangeEvent;
use crate::configuration::{ConfigurationManager, TestRunnerImplementation};
use crate::coverage::{compute_coverage, MockComputeCoverage};
use crate::test_execution::{TestRunError, TestRunHandler, TestRunProcessor, change_event_thread, mock_run_tests, stub_parse_output, test_run_thread};
use crate::test_helpers::test_name::test_name;
use crate::test_helpers::test_run_setup::TestRunSetup;
use crate::test_run_model::{FailedTestRun, SingleTestStatus, TestId, TestRunState};

#[test]
#[cfg(target_os = "windows")]
pub fn handle_single_test_run()
{
    let (test_run_tx, test_run_rx) = Tx::new();

    let mut handler = TestRunSetup::builder(TestRunnerImplementation::Nextest, test_name(function_name!()), "simple_project")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let state = test_run_rx.last().unwrap();
    assert!(state.tests.iter().all(|test| { test.status == SingleTestStatus::Passed }));

    let test_to_run_again = state.tests.iter().find(|test| test.name == "add_2_and_2_is_4").unwrap();

    handler.handle(
        ChangeEvent::SingleTest {
            id: test_to_run_again.id(),
            update_snapshots: false
        },
        Cancellation::default()
    );

    let running_single = test_run_rx.next().unwrap();

    assert_that!(&running_single.state, is_variant!(TestRunState::Running));

    // Assert that all tests are still passing, except the single test, which is running
    assert!(
        state
            .tests
            .iter()
            .all(|test| { (test.id() == test_to_run_again.id() && test.status == SingleTestStatus::Unknown) || test.status == SingleTestStatus::Passed })
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

    let mut handler = TestRunSetup::builder(TestRunnerImplementation::Nextest, test_name(function_name!()), "simple_project")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let all_tests = test_run_rx.last().unwrap();

    let pinned_test = all_tests.tests.iter().next().unwrap().to_owned();

    handler.handle(ChangeEvent::PinTest { id: pinned_test.id() }, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let pinned_run = test_run_rx.last().unwrap();
    // Assert that all tests are unknown, except the pinned test, which is passing
    assert!(
        pinned_run
            .tests
            .iter()
            .all(|test| { (test.id() == pinned_test.id() && test.status == SingleTestStatus::Passed) || test.status == SingleTestStatus::Unknown })
    );
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_is_unpinned_all_tests_are_run_when_changes_are_handled()
{
    let (test_run_tx, test_run_rx) = Tx::new();
    let mut handler = TestRunSetup::builder(TestRunnerImplementation::Nextest, test_name(function_name!()), "simple_project")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let all_tests = test_run_rx.last().unwrap();

    let pinned_test = all_tests.tests.iter().next().unwrap().to_owned();

    handler.handle(ChangeEvent::PinTest { id: pinned_test.id() }, Cancellation::default());
    handler.handle(ChangeEvent::ClearPinnedTests, Cancellation::default());
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let test_run = test_run_rx.last().unwrap();
    // Assert that all tests are unknown, except the pinned test, which is passing
    assert!(test_run.tests.iter().all(|test| { test.status == SingleTestStatus::Passed }));
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_snapshot_test_is_run_with_update_snapshots_enabled_replace_new_snapshot_with_approved() -> Result<(), IoError>
{
    let setup = TestRunSetup::builder(TestRunnerImplementation::Nextest, test_name(function_name!()), "simple_project")
        .build()
        .clean_snapshots();

    let expected_approved_snapshot = setup.get_snapshots_path().join("example_snapshot.png");
    let mut handler = setup.build_test_run_handler();

    // Run all tests first to generate a new snapshot
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let snapshot_test_id = TestId::new("snapshot_test".to_string());

    handler.handle(
        ChangeEvent::SingleTest {
            id: snapshot_test_id,
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

    let mut handler = TestRunSetup::builder(TestRunnerImplementation::Nextest, test_name(function_name!()), "simple_project_failing_tests")
        .tests_status_sender(test_run_tx)
        .build()
        .clean_output()
        .build_test_run_handler();

    // Run all tests first to generate a new snapshot
    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let failed_test = TestId::new("multiply_2_and_2_is_4".to_string());

    let state = test_run_rx.last().unwrap();

    let failed_test = state.tests.find(&failed_test).unwrap();
    assert_that!(
        &failed_test.output,
        contains_in_order(vec![
            "thread 'multiply_2_and_2_is_4' panicked at tests\\multiply_tests.rs:6:5:".to_string(),
            "assertion `left == right` failed".to_string(),
            "left: 5".to_string(),
            "right: 4".to_string(),
            "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace".to_string()
        ])
    );

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
pub fn when_test_run_fails_error_is_reported()
{
    let mut run_tests = mock_run_tests();

    run_tests.expect_run_tests().returning(|_, _, _| Err(TestRunError::Cancelled(Cancelled)));

    let processor = TestRunProcessor::new(run_tests, stub_parse_output());
    let (tests_sender, tests_receiver) = Tx::new();

    let mut handler = TestRunHandler::builder()
        .runner(processor)
        .coverage(Box::new(compute_coverage::stub()))
        .tests_status_sender(tests_sender)
        .coverage_status_sender(Tx::stub())
        .log(Tx::stub())
        .configuration(ConfigurationManager::default_config(Tx::stub(), Tx::stub()))
        .build();

    handler.handle(ChangeEvent::DefaultRun, Cancellation::default());

    let last = tests_receiver.last().unwrap().state;

    assert_eq!(
        last,
        TestRunState::Failed(FailedTestRun {
            inner_error_display: "test run cancelled".to_string()
        })
    );
}

#[test]
pub fn when_configuration_changes_tests_are_run()
{
    let mut run_tests = mock_run_tests();

    // This expectation of 'once()' is the test assertion
    run_tests.expect_run_tests().once().returning(|_, _, _| Err(TestRunError::Cancelled(Cancelled)));
    run_tests.expect_run_test().returning(|_, _, _, _| Err(TestRunError::Cancelled(Cancelled)));

    let test_run_processor = TestRunProcessor::new(run_tests, stub_parse_output());
    let (change_event_tx, change_event_rx) = Tx::new();
    let (test_run_tx, test_run_rx) = Tx::new();

    let handler = TestRunHandler::builder()
        .runner(test_run_processor)
        .tests_status_sender(Tx::stub())
        .coverage(Box::new(compute_coverage::stub()))
        .coverage_status_sender(Tx::stub())
        .configuration(ConfigurationManager::default_config(Tx::stub(), Tx::stub()))
        .log(Tx::stub())
        .build();

    let change_event_thread = change_event_thread(change_event_rx, test_run_tx);
    let test_run_thread = test_run_thread(test_run_rx, handler);

    let configuration = ConfigurationManager::default_config(Tx::stub(), change_event_tx);
    change_configuration_to_trigger_test_run(configuration);

    _ = test_run_thread.join().expect("failed to join");
    change_event_thread.join().expect("failed to join");
}

fn change_configuration_to_trigger_test_run(mut configuration: ConfigurationManager)
{
    configuration.update(|c| c.coverage_enabled = true);
}
