use std::thread::{self, JoinHandle};

use bon::Builder;
use crossbeam_channel::Receiver;
use passivate_delegation::{ActorEvent, BTx, Cancellation, Handler, Tx};

use super::TestRunProcessor;
use crate::change_events::ChangeEvent;
use crate::configuration::ConfigurationManager;
use crate::coverage::{BComputeCoverage, ComputeCoverage, CoverageStatus};
use crate::cross_cutting::{BLog, Log};
use crate::test_run_model::{FailedTestRun, TestId, TestRun};

#[mockall::automock]
pub trait ProvideBool
{
    fn get(&self) -> bool;
}

pub struct GetBool
{
    value: bool
}

impl GetBool
{
    pub fn new(value: bool) -> Self
    {
        Self { value }
    }
}

impl ProvideBool for GetBool
{
    fn get(&self) -> bool
    {
        self.value
    }
}

pub fn test_run_thread(
    rx: Receiver<ActorEvent<ChangeEvent>>,
    mut handler: TestRunHandler
) -> JoinHandle<TestRunHandler>
{
    thread::spawn(move || {
        while let Ok(event) = rx.recv()
        {
            handler.handle(event.event, event.cancellation);
        }

        handler
    })
}

#[derive(Builder)]
pub struct TestRunHandler
{
    runner: TestRunProcessor,
    coverage: BComputeCoverage,
    tests_status_sender: BTx<TestRun>,
    coverage_status_sender: BTx<CoverageStatus>,
    log: BLog,
    configuration: ConfigurationManager,
    pinned_test: Option<TestId>
}

impl TestRunHandler
{
    pub fn handle(&mut self, event: ChangeEvent, cancellation: Cancellation)
    {
        match event
        {
            ChangeEvent::DefaultRun => self.run_tests(cancellation.clone()),
            ChangeEvent::PinTest { id } =>
            {
                self.pinned_test = Some(id);
                self.run_tests(cancellation.clone());
            }
            ChangeEvent::ClearPinnedTests =>
            {
                self.pinned_test = None;
                self.run_tests(cancellation.clone());
            }
            ChangeEvent::SingleTest { id, update_snapshots } => self.run_test(&id, update_snapshots, cancellation.clone())
        }
    }

    fn run_tests(&mut self, cancellation: Cancellation)
    {
        if let Some(pinned_test) = self.pinned_test.clone()
        {
            let update_snapshots = false;
            self.run_test(&pinned_test, update_snapshots, cancellation.clone());
            return;
        }

        let coverage_enabled = self.coverage_enabled();

        if coverage_enabled
        {
            self.coverage_status_sender.send(CoverageStatus::Preparing);
        }

        if let Err(clean_error) = self.coverage.clean_coverage_output()
        {
            self.log.info(&format!("error cleaning coverage output: {:?}", clean_error));
        }

        if cancellation.is_cancelled()
        {
            return;
        }

        let test_output = self.runner.run_tests(&mut self.tests_status_sender, coverage_enabled, cancellation.clone());

        if cancellation.is_cancelled()
        {
            return;
        }

        match test_output
        {
            Ok(_) =>
            {
                if self.coverage_enabled()
                {
                    self.log.info("Coverage enabled, computing...");
                    self.compute_coverage(cancellation.clone());
                }
                else
                {
                    self.log.info("Coverage disabled.");
                }
            }
            Err(test_error) =>
            {
                let error_status = FailedTestRun {
                    inner_error_display: test_error.to_string()
                };
                self.tests_status_sender.send(TestRun::from_failed(error_status));
            }
        };
    }

    fn compute_coverage(&mut self, cancellation: Cancellation)
    {
        self.coverage_status_sender.send(CoverageStatus::Running);

        let coverage_status = self.coverage.compute_coverage(cancellation.clone());

        self.log.info("Coverage completed.");

        match coverage_status
        {
            Ok(coverage_status) => self.coverage_status_sender.send(coverage_status),
            Err(coverage_error) => self.coverage_status_sender.send(CoverageStatus::Error(coverage_error.to_string()))
        }
    }

    fn run_test(&mut self, id: &TestId, update_snapshots: bool, cancellation: Cancellation)
    {
        let result = self.runner.run_test(&mut self.tests_status_sender, id, update_snapshots, cancellation);

        if let Err(error) = result
        {
            let error_status = FailedTestRun {
                inner_error_display: error.to_string()
            };
            self.tests_status_sender.send(TestRun::from_failed(error_status));
        }
    }

    pub fn coverage_enabled(&self) -> bool
    {
        self.coverage_enabled.get()
    }
}
