use std::sync::mpsc::Sender;

use crate::{coverage::stub_compute_coverage, cross_cutting::stub_log, test_execution::{stub_parse_output, stub_run_tests, TestRunHandler, TestRunProcessor}, test_run_model::TestRun};

use super::channel_fakes::stub_sender;

pub fn stub() -> TestRunHandler {
    let test_run_processor = TestRunProcessor::new(stub_run_tests(), stub_parse_output(), stub_log());
    let coverage_enabled = false;
    TestRunHandler::new(
        test_run_processor, 
        stub_compute_coverage(), 
        stub_sender(), 
        stub_sender(), 
        stub_log(),
        coverage_enabled)
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_sender: Sender<TestRun>) -> TestRunHandler {
    let coverage_enabled = false;
    TestRunHandler::new(
        test_run_processor, 
        stub_compute_coverage(), 
        tests_sender, 
        stub_sender(), 
        stub_log(),
        coverage_enabled)
}