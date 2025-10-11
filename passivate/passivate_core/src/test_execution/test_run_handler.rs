use std::thread::{self, JoinHandle};

use bon::Builder;
use camino::Utf8PathBuf;
use passivate_delegation::{CancellableMessage, Cancellation, Rx, Tx};
use passivate_hyp_names::hyp_id::HypId;

use crate::change_events::ChangeEvent;
use crate::configuration::ConfigurationManager;
use crate::coverage::{ComputeCoverage, CoverageStatus};
use crate::cross_cutting::LogEvent;
use crate::test_execution::TestRunner;
use crate::test_run_model::{FailedTestRun, TestRun};

pub fn test_run_thread(rx: Rx<CancellableMessage<ChangeEvent>>, mut handler: TestRunHandler) -> JoinHandle<TestRunHandler>
{
    thread::spawn(move || {
        while let Ok(event) = rx.recv()
        {
            handler.handle(event.message, event.cancellation);
        }

        handler
    })
}

#[derive(Builder)]
pub struct TestRunHandler
{
    runner: TestRunner,
    coverage: Box<dyn ComputeCoverage + Send>,
    tests_status_sender: Tx<TestRun>,
    coverage_status_sender: Tx<CoverageStatus>,
    log: Tx<LogEvent>,
    configuration: ConfigurationManager,
    pinned_hyp: Option<HypId>
}

impl TestRunHandler
{
    pub fn handle(&mut self, event: ChangeEvent, cancellation: Cancellation)
    {
        match event
        {
            ChangeEvent::DefaultRun => self.run_hyps(cancellation.clone()),
            ChangeEvent::PinHyp { id } =>
            {
                self.pinned_hyp = Some(id);
                self.run_hyps(cancellation.clone());
            }
            ChangeEvent::ClearPinnedHyps =>
            {
                self.pinned_hyp = None;
                self.run_hyps(cancellation.clone());
            }
            ChangeEvent::SingleHyp { id, update_snapshots } => self.run_hyp(&id, update_snapshots, cancellation.clone())
        }
    }

    fn run_hyps(&mut self, cancellation: Cancellation)
    {
        if let Some(pinned_hyp) = self.pinned_hyp.clone()
        {
            let update_snapshots = false;
            self.run_hyp(&pinned_hyp, update_snapshots, cancellation.clone());
            return;
        }

        let coverage_enabled = self.coverage_enabled();

        if coverage_enabled
        {
            self.coverage_status_sender.send(CoverageStatus::Preparing);
        }

        if let Err(clean_error) = self.coverage.clean_coverage_output()
        {
            self.log.send(LogEvent::new(&format!("error cleaning coverage output: {:?}", clean_error)));
        }

        if cancellation.is_cancelled()
        {
            return;
        }

        let snapshot_path = self.get_snapshots_path();
           
        let test_output = self.runner.run_hyps(coverage_enabled, cancellation.clone(), &mut self.tests_status_sender, Vec::new(), snapshot_path);

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
                    self.log.send(LogEvent::new("Coverage enabled, computing..."));
                    self.compute_coverage(cancellation.clone());
                }
                else
                {
                    self.log.send(LogEvent::new("Coverage disabled."));
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

    fn get_snapshots_path(&self) -> Option<Utf8PathBuf> {
        self.configuration.get(|c| c.snapshots_path.clone())
            .map(Utf8PathBuf::try_from)
            .map_or_else(|| None, |path|
            {
                path.map_or_else(|_|
                {
                    self.log.send(LogEvent::new("snapshot path was not a valid UTF8 path"));
                    None
                }, 
                |p|
                {
                    Some(p)
                })
            })
    }
    
    fn compute_coverage(&mut self, cancellation: Cancellation)
    {
        self.coverage_status_sender.send(CoverageStatus::Running);

        let coverage_status = self.coverage.compute_coverage(cancellation.clone());

        self.log.send(LogEvent::new("Coverage completed."));

        match coverage_status
        {
            Ok(coverage_status) => self.coverage_status_sender.send(coverage_status),
            Err(coverage_error) => self.coverage_status_sender.send(CoverageStatus::Error(coverage_error.to_string()))
        }
    }

    fn run_hyp(&mut self, id: &HypId, update_snapshots: bool, cancellation: Cancellation)
    {
        let snapshot_path = self.get_snapshots_path();
        let result = self.runner.run_hyp(id, update_snapshots, cancellation, &mut self.tests_status_sender, snapshot_path);

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
        self.configuration.get(|c| c.coverage_enabled)
    }
}
