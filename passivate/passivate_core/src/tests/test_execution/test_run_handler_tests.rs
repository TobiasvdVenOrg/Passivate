use std::sync::mpsc::channel;
use crate::actors::{Cancellation, Cancelled, Handler};
use crate::cross_cutting::stub_log;
use crate::test_execution::{mock_run_tests, stub_parse_output, TestRunError, TestRunProcessor};
use crate::test_helpers::builder::nextest_builder;
use crate::test_helpers::fakes::test_run_handler_fakes;
use crate::test_run_model::{FailedTestRun, TestId, TestRunState};
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

    
    handler.handle(ChangeEvent::SingleTest {
        id: TestId::new("".to_string()),
        update_snapshots: false
    }, Cancellation::default());

    let last = test_run_receiver.try_iter().last().unwrap();

    assert_that!(&last.state, is_variant!(TestRunState::Idle));
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
