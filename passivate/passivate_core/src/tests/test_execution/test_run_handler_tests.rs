use std::sync::mpsc::channel;
use crate::actors::{Cancellation, Cancelled, Handler};
use crate::cross_cutting::stub_log;
use crate::test_execution::{mock_run_tests, stub_parse_output, TestRunError, TestRunProcessor};
use crate::test_helpers::fakes::test_run_handler_fakes;
use crate::test_run_model::{FailedTestRun, TestRunState};
use crate::change_events::ChangeEvent;
use pretty_assertions::assert_eq;

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
