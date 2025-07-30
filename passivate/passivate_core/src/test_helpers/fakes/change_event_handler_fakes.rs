use passivate_delegation::{Actor, ActorTx, Tx};

use super::test_run_actor_fakes;
use crate::test_execution::{ChangeEventHandler, TestRunProcessor};
use crate::test_run_model::TestRun;

pub fn stub() -> ChangeEventHandler
{
    ChangeEventHandler::new(ActorTx::stub())
}

pub fn stub_with_test_run_processor_and_tests_sender(test_run_processor: TestRunProcessor, tests_tx: Tx<TestRun>) -> ChangeEventHandler
{
    let test_run_handler = test_run_actor_fakes::stub_with_test_run_processor_and_tests_sender(test_run_processor, tests_tx);

    let (_actor, actor_tx) = Actor::new(test_run_handler);
    ChangeEventHandler::new(actor_tx)
}
