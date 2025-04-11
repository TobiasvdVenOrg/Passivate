use crate::{coverage::stub_compute_coverage, cross_cutting::stub_log, delegation::{stub_give, Give}, test_execution::{stub_parse_output, stub_run_tests, TestRunHandler, TestRunProcessor}, test_run_model::TestRun};

pub fn stub() -> TestRunHandler {
    let test_run_processor = TestRunProcessor::new(stub_run_tests(), stub_parse_output());
    let coverage_enabled = false;
    TestRunHandler::new(
        test_run_processor, 
        stub_compute_coverage(), 
        stub_give(), 
        stub_give(), 
        stub_log(),
        coverage_enabled)
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_sender: Box<dyn Give<TestRun>>) -> TestRunHandler {
    let coverage_enabled = false;
    TestRunHandler::new(
        test_run_processor, 
        stub_compute_coverage(), 
        tests_sender, 
        stub_give(), 
        stub_log(),
        coverage_enabled)
}