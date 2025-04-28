use crate::{change_events::ChangeEvent, coverage::{ComputeCoverage, CoverageStatus}, cross_cutting::Log, test_run_model::TestRun};
use passivate_delegation::{Actor, ActorTx, Tx};

use super::{TestRunHandler, TestRunProcessor};


pub struct TestRunActor {
    actor: Actor<ChangeEvent, TestRunHandler>
}

impl TestRunActor {
    pub fn new(
        runner: TestRunProcessor,
        coverage: Box<dyn ComputeCoverage + Send>, 
        tests_status_sender: Tx<TestRun>,
        coverage_status_sender: Tx<CoverageStatus>,
        log: Box<dyn Log + Send>,
        coverage_enabled: bool) -> (ActorTx<ChangeEvent>, Self) {
        let handler = TestRunHandler::new(
            runner, 
            coverage, 
            tests_status_sender,
            coverage_status_sender,
            log,
            coverage_enabled
            );

        let (tx, actor) = Actor::new(handler);

        ( tx, Self { actor } )
    }

    pub fn into_inner(&mut self) -> TestRunHandler {
        self.actor.into_inner()
    }
}