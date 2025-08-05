use crossbeam_channel::Receiver;
use passivate_delegation::{Actor, ActorEvent, Tx};

use super::{TestRunHandler, TestRunProcessor};
use crate::change_events::ChangeEvent;
use crate::coverage::{ComputeCoverage, CoverageStatus};
use crate::cross_cutting::Log;
use crate::test_run_model::TestRun;

pub struct TestRunActor<TCoverageEnabled>
where
    TCoverageEnabled: Fn() -> bool + Send + 'static
{
    actor: Actor<ChangeEvent, TestRunHandler<TCoverageEnabled>>
}

impl<TCoverageEnabled> TestRunActor<TCoverageEnabled>
where
    TCoverageEnabled: Fn() -> bool + Send + 'static
{
    pub fn new(
        rx: Receiver<ActorEvent<ChangeEvent>>,
        runner: TestRunProcessor,
        coverage: Box<dyn ComputeCoverage + Send>,
        tests_status_sender: Tx<TestRun>,
        coverage_status_sender: Tx<CoverageStatus>,
        log: Box<dyn Log>,
        coverage_enabled: TCoverageEnabled
    ) -> Self
    {
        let handler = TestRunHandler::new(runner, coverage, tests_status_sender, coverage_status_sender, log, coverage_enabled);

        Self { actor: Actor::with_rx(handler, rx, String::from("test_run_actor")) }
    }

    pub fn into_inner(&mut self) -> TestRunHandler<TCoverageEnabled>
    {
        self.actor.into_inner()
    }
}
