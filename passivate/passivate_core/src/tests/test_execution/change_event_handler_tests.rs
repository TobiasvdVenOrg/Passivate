use std::sync::mpsc::channel;
use crate::actors::{Cancellation, Handler};
use crate::cross_cutting::stub_log;
use crate::test_execution::{mock_run_tests, stub_parse_output, ChangeEventHandler, TestRunError, TestRunProcessor};
use crate::test_run_model::{FailedTestRun, TestRunState};
use crate::coverage::stub_compute_coverage;
use crate::change_events::ChangeEvent;
use pretty_assertions::assert_eq;

#[test]
pub fn when_test_run_fails_error_is_reported() {  
    let mut run_tests = mock_run_tests();

    run_tests.expect_run_tests()
        .returning(|_| { Err(TestRunError::Io("example error".to_string())) });

    let processor = TestRunProcessor::new(run_tests, stub_parse_output(), stub_log());
    let (tests_sender, tests_receiver) = channel();
    let (coverage_sender, _coverage_receiver) = channel();
    let mut handler = ChangeEventHandler::new(processor, stub_compute_coverage(), tests_sender, coverage_sender, stub_log());

    handler.handle(ChangeEvent::File, Cancellation::default());

    let last = tests_receiver.try_iter().last().unwrap().state;

    assert_eq!(last, TestRunState::Failed(FailedTestRun { inner_error_display: "example error".to_string() }));
}
