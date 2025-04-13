use crate::{delegation::{Actor, ActorTx, Tx}, test_execution::{ChangeEventHandler, TestRunProcessor}, test_run_model::TestRun};

use super::test_run_handler_fakes;

pub fn stub() -> ChangeEventHandler {
    ChangeEventHandler::new(ActorTx::stub())
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_tx: Tx<TestRun>) -> ChangeEventHandler {
    let test_run_handler = test_run_handler_fakes::stub_with_test_run_processor_and_tests_sender(test_run_processor, tests_tx);

    let (actor_tx, _actor) = Actor::new(test_run_handler);
    ChangeEventHandler::new(actor_tx)
}
