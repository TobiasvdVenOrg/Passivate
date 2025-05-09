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
        log: Box<dyn Log>,
        coverage_enabled: bool) -> (Self, ActorTx<ChangeEvent>) {
        let handler = TestRunHandler::new(
            runner, 
            coverage, 
            tests_status_sender,
            coverage_status_sender,
            log,
            coverage_enabled
            );

        let (actor, tx) = Actor::new(handler);

        ( Self { actor }, tx )
    }

    pub fn into_inner(&mut self) -> TestRunHandler {
        self.actor.into_inner()
    }
}