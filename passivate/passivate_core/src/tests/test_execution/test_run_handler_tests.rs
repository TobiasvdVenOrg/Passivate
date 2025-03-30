use std::sync::mpsc::channel;
use crate::actors::{Cancellation, Cancelled, Handler};
use crate::cross_cutting::stub_log;
use crate::test_execution::{mock_run_tests, stub_parse_output, TestRunError, TestRunProcessor};
use crate::test_helpers::builder::nextest_builder;
use crate::test_helpers::fakes::test_run_handler_fakes;
use crate::test_run_model::{FailedTestRun, SingleTestStatus, TestRunState};
use crate::change_events::ChangeEvent;
use galvanic_assert::{assert_that, is_variant};
use pretty_assertions::assert_eq;
use stdext::function_name;

#[test]
pub fn handle_single_test_run() {
    let (test_run_sender, test_run_receiver) = channel();
    let mut handler = nextest_builder()
        .with_workspace("simple_project")
        .with_output(function_name!())
        .receive_tests_status(test_run_sender)
        .clean_output()
        .build();

    // Run all tests first, single test running currently relies on knowing the test id of an existing test
    handler.handle(ChangeEvent::File, Cancellation::default());

    let state = test_run_receiver.try_iter().last().unwrap();
    assert!(state.tests.iter().all(|test| {
        test.status == SingleTestStatus::Passed
    }));

    let test_to_run_again = state.tests.iter().find(|test| test.name == "add_2_and_2_is_4").unwrap();

    handler.handle(ChangeEvent::SingleTest {
        id: test_to_run_again.id(),
        update_snapshots: false
    }, Cancellation::default());

    let running_single = test_run_receiver.try_iter().next().unwrap();

    assert_that!(&running_single.state, is_variant!(TestRunState::Running));

    // Assert that all tests are still passing, except the single test, which is running
    assert!(state.tests.iter().all(|test| {
        (test.id() == test_to_run_again.id() && test.status == SingleTestStatus::Unknown) 
        ||
        test.status == SingleTestStatus::Passed
    }));

    let final_state = test_run_receiver.try_iter().last().unwrap();

    assert_that!(&final_state.state, is_variant!(TestRunState::Idle));
    assert!(final_state.tests.iter().all(|test| {
        test.status == SingleTestStatus::Passed
    }));
}

#[test]
pub fn when_test_run_fails_error_is_reported() {  
    let mut run_tests = mock_run_tests();

    run_tests.expect_run_tests()
        .returning(|_, _, _| { Err(TestRunError::Cancelled(Cancelled)) });

    let processor = TestRunProcessor::new(run_tests, stub_parse_output(), stub_log());
    let (tests_sender, tests_receiver) = channel();
    let mut handler = test_run_handler_fakes::stub_with_test_run_processor_and_tests_sender(processor, tests_sender);

    handler.handle(ChangeEvent::File, Cancellation::default());

    let last = tests_receiver.try_iter().last().unwrap().state;

    assert_eq!(last, TestRunState::Failed(FailedTestRun { inner_error_display: "test run cancelled".to_string() }));
}
