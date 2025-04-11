use crate::{delegation::{stub_loan, Actor, Give}, test_execution::{ChangeEventHandler, TestRunProcessor}, test_run_model::TestRun};

use super::test_run_handler_fakes;

pub fn stub() -> ChangeEventHandler {
    ChangeEventHandler::new(stub_loan())
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_sender: Box<dyn Give<TestRun>>) -> ChangeEventHandler {
    let test_run_handler = test_run_handler_fakes::stub_with_test_run_processor_and_tests_sender(test_run_processor, tests_sender);

    let test_run_actor = Actor::new(test_run_handler);
    ChangeEventHandler::new(Box::new(test_run_actor.loan()))
}
