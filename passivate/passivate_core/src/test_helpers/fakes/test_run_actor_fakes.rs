use crossbeam_channel::Receiver;
use passivate_delegation::{ActorEvent, Tx};

use crate::change_events::ChangeEvent;
use crate::coverage::stub_compute_coverage;
use crate::cross_cutting::stub_log;
use crate::test_execution::{TestRunActor, TestRunHandler, TestRunProcessor, stub_parse_output, stub_run_tests};
use crate::test_run_model::TestRun;

pub fn stub(rx: Receiver<ActorEvent<ChangeEvent>>) -> TestRunActor<impl Fn() -> bool>
{
    let test_run_processor = TestRunProcessor::new(stub_run_tests(), stub_parse_output());
    let coverage_enabled = || false;
    TestRunActor::new(rx, test_run_processor, stub_compute_coverage(), Tx::stub(), Tx::stub(), stub_log(), coverage_enabled)
}

pub fn stub_with_test_run_processor_and_tests_sender(rx: Receiver<ActorEvent<ChangeEvent>>, test_run_processor: TestRunProcessor, tests_tx: Tx<TestRun>) -> TestRunActor<impl Fn() -> bool>
{
    let coverage_enabled = || false;
    TestRunActor::new(rx, test_run_processor, stub_compute_coverage(), tests_tx, Tx::stub(), stub_log(), coverage_enabled)
}

pub fn stub_with_coverage_enabled(coverage_enabled: impl Fn() -> bool) -> TestRunHandler<impl Fn() -> bool>
{
    let test_run_processor = TestRunProcessor::new(stub_run_tests(), stub_parse_output());
    TestRunHandler::new(
        test_run_processor,
        stub_compute_coverage(),
        Tx::stub(),
        Tx::stub(),
        stub_log(),
        coverage_enabled
    )
}