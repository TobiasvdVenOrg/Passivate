use std::sync::mpsc::Sender;

use crate::{delegation::Actor, cross_cutting::stub_log, test_execution::{ChangeEventHandler, TestRunProcessor}, test_run_model::TestRun};

use super::test_run_handler_fakes;


pub fn stub() -> ChangeEventHandler {
    let test_run_handler = test_run_handler_fakes::stub();

    let test_run_actor = Actor::new(test_run_handler);
    ChangeEventHandler::new(test_run_actor.api(), stub_log())
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_sender: Sender<TestRun>) -> ChangeEventHandler {
    let test_run_handler = test_run_handler_fakes::stub_with_test_run_processor_and_tests_sender(test_run_processor, tests_sender);

    let test_run_actor = Actor::new(test_run_handler);
    ChangeEventHandler::new(test_run_actor.api(), stub_log())
}