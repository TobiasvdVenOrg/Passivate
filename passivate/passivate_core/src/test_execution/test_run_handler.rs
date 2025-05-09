use crate::{change_events::ChangeEvent, coverage::{ComputeCoverage, CoverageStatus}};
use crate::test_run_model::{FailedTestRun, TestId, TestRun};
use passivate_delegation::{Cancellation, Tx, Handler};
use crate::cross_cutting::Log;

use super::TestRunProcessor;

pub struct TestRunHandler {
    runner: TestRunProcessor,
    coverage: Box<dyn ComputeCoverage + Send>, 
    tests_status_sender: Tx<TestRun>,
    coverage_status_sender: Tx<CoverageStatus>,
    log: Box<dyn Log>,
    coverage_enabled: bool,
    pinned_test: Option<TestId>
}

impl TestRunHandler {
    pub fn new(
        runner: TestRunProcessor,
        coverage: Box<dyn ComputeCoverage + Send>, 
        tests_status_sender: Tx<TestRun>,
        coverage_status_sender: Tx<CoverageStatus>,
        log: Box<dyn Log>,
        coverage_enabled: bool) -> Self {
            Self {
            runner, 
            coverage, 
            tests_status_sender,
            coverage_status_sender,
            log,
            coverage_enabled,
            pinned_test: None
        }
    }

    fn run_tests(&mut self, cancellation: Cancellation) {
        if let Some(pinned_test) = self.pinned_test.clone() {
            let update_snapshots = false;
            self.run_test(&pinned_test, update_snapshots, cancellation.clone());
            return;
        }

        if self.coverage_enabled {
            self.coverage_status_sender.send(CoverageStatus::Preparing);
        }

        if let Err(clean_error) = self.coverage.clean_coverage_output() {
            self.log.info(&format!("error cleaning coverage output: {:?}", clean_error));
        }

        if cancellation.is_cancelled() { return }

        let test_output = self.runner.run_tests(&mut self.tests_status_sender, self.coverage_enabled, cancellation.clone());

        if cancellation.is_cancelled() { return }

        match test_output {
            Ok(_) => {
                if self.coverage_enabled {
                    self.log.info("Coverage enabled, computing...");
                    self.compute_coverage(cancellation.clone());
                } else {
                    self.log.info("Coverage disabled.");
                }
            },
            Err(test_error) => {
                let error_status = FailedTestRun { inner_error_display: test_error.to_string() };
                self.tests_status_sender.send(TestRun::from_failed(error_status));
            }
        };
    }

    fn compute_coverage(&mut self, cancellation: Cancellation) {
        self.coverage_status_sender.send(CoverageStatus::Running);

        let coverage_status = self.coverage.compute_coverage(cancellation.clone());

        self.log.info("Coverage completed.");

        match coverage_status {
            Ok(coverage_status) => self.coverage_status_sender.send(coverage_status),
            Err(coverage_error) => {
                self.coverage_status_sender.send(CoverageStatus::Error(coverage_error.to_string()))
            }
        }
    }

    pub fn coverage_enabled(&self) -> bool { self.coverage_enabled }
    
    fn run_test(&mut self, id: &TestId, update_snapshots: bool, cancellation: Cancellation) {
        let result = self.runner.run_test(&mut self.tests_status_sender, id, update_snapshots, cancellation);

        if let Err(error) = result {
            let error_status = FailedTestRun { inner_error_display: error.to_string() };
            self.tests_status_sender.send(TestRun::from_failed(error_status));
        }
    }
}

impl Handler<ChangeEvent> for TestRunHandler {
    fn handle(&mut self, event: ChangeEvent, cancellation: Cancellation) {
        match event {
            ChangeEvent::File => self.run_tests(cancellation.clone()),
            ChangeEvent::Configuration(configuration) => {
                        let coverage_changed = self.coverage_enabled != configuration.new.coverage_enabled;
                        self.coverage_enabled = configuration.new.coverage_enabled;
                        
                        if coverage_changed && !self.coverage_enabled {
                            self.coverage_status_sender.send(CoverageStatus::Disabled);
                        }

                        self.run_tests(cancellation.clone());
                    },
            ChangeEvent::PinTest { id } => {
                self.pinned_test = Some(id);
                self.run_tests(cancellation.clone());
            },
            ChangeEvent::ClearPinnedTests => {
                self.pinned_test = None;
                self.run_tests(cancellation.clone());
            }
            ChangeEvent::SingleTest { id, update_snapshots } => self.run_test(&id, update_snapshots, cancellation.clone()),
        }
    }
}