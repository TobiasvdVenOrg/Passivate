use crate::{change_events::ChangeEvent, coverage::stub_compute_coverage, cross_cutting::stub_log, test_run_model::TestRun};
use crate::test_execution::{stub_parse_output, stub_run_tests, TestRunActor, TestRunHandler, TestRunProcessor};
use passivate_delegation::{ActorTx, Tx};

pub fn stub() -> (TestRunActor<impl Fn() -> bool>, ActorTx<ChangeEvent>) {
    let test_run_processor = TestRunProcessor::new(stub_run_tests(), stub_parse_output());
    let coverage_enabled = || false;
    TestRunActor::new(
        test_run_processor, 
        stub_compute_coverage(), 
        Tx::stub(), 
        Tx::stub(), 
        stub_log(),
        coverage_enabled)
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_tx: Tx<TestRun>) -> TestRunHandler<impl Fn() -> bool> {
    let coverage_enabled = || false;
    TestRunHandler::new(
        test_run_processor, 
        stub_compute_coverage(), 
        tests_tx, 
        Tx::stub(), 
        stub_log(),
        coverage_enabled)
}