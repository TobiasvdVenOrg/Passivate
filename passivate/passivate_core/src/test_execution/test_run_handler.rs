use std::sync::mpsc::Sender;

use crate::{actors::{Cancellation, Handler}, change_events::ChangeEvent, coverage::{ComputeCoverage, CoverageStatus}, cross_cutting::Log, test_run_model::{FailedTestRun, TestRun}};

use super::TestRunProcessor;

pub struct TestRunHandler {
    runner: TestRunProcessor,
    coverage: Box<dyn ComputeCoverage + Send>, 
    tests_status_sender: Sender<TestRun>,
    coverage_status_sender: Sender<CoverageStatus>,
    log: Box<dyn Log + Send>,
    coverage_enabled: bool
}

impl TestRunHandler {
    pub fn new(
        runner: TestRunProcessor,
        coverage: Box<dyn ComputeCoverage + Send>, 
        tests_status_sender: Sender<TestRun>,
        coverage_status_sender: Sender<CoverageStatus>,
        log: Box<dyn Log + Send>,
        coverage_enabled: bool) -> Self {
            Self {
            runner, 
            coverage, 
            tests_status_sender,
            coverage_status_sender,
            log,
            coverage_enabled
        }
    }

    fn run_tests(&mut self, cancellation: Cancellation) {
        if self.coverage_enabled {
            let _ = self.coverage_status_sender.send(CoverageStatus::Preparing);
        }

        if let Err(clean_error) = self.coverage.clean_coverage_output() {
            self.log.info(&format!("ERROR CLEANING: {:?}", clean_error));
        }

        self.log.info("Done cleaning it!");
        if cancellation.is_cancelled() { return }

        let test_output = self.runner.run_tests(&self.tests_status_sender, self.coverage_enabled, cancellation.clone());

        self.log.info("Done running it!");

        if cancellation.is_cancelled() { return }

        match test_output {
            Ok(_) => {
                if self.coverage_enabled {
                    self.log.info("Coverage enabled!");
                    self.compute_coverage(cancellation.clone());
                } else {
                    self.log.info("Coverage disabled!");
                }
            },
            Err(test_error) => {
                let error_status = FailedTestRun { inner_error_display: test_error.to_string() };
                let _  = self.tests_status_sender.send(TestRun::from_failed(error_status));
            }
        };
    }

    fn compute_coverage(&self, cancellation: Cancellation) {
        let _ = self.coverage_status_sender.send(CoverageStatus::Running);

        let coverage_status = self.coverage.compute_coverage(cancellation.clone());

        self.log.info("Done covering it!");

        let _ = match coverage_status {
            Ok(coverage_status) => self.coverage_status_sender.send(coverage_status),
            Err(coverage_error) => {
                self.coverage_status_sender.send(CoverageStatus::Error(coverage_error.to_string()))
            }
        };
    }

    pub fn coverage_enabled(&self) -> bool { self.coverage_enabled }
    
    fn run_test(&mut self, id: crate::test_run_model::TestId, update_snapshots: bool, cancellation: Cancellation) {
        let result = self.runner.run_test(&self.tests_status_sender, id, update_snapshots, cancellation);

        if let Err(error) = result {
            let error_status = FailedTestRun { inner_error_display: error.to_string() };
            let _  = self.tests_status_sender.send(TestRun::from_failed(error_status));
        }
    }
}

impl Handler<ChangeEvent> for TestRunHandler {
    fn handle(&mut self, event: ChangeEvent, cancellation: Cancellation) {
        self.log.info("Handling it!");

        match event {
            ChangeEvent::File => self.run_tests(cancellation.clone()),
            ChangeEvent::Configuration(passivate_config) => {
                        let configuration_changed = self.coverage_enabled != passivate_config.coverage_enabled;
                        self.coverage_enabled = passivate_config.coverage_enabled;

                        if self.coverage_enabled {
                            if configuration_changed {
                                self.run_tests(cancellation.clone());
                            }
                        } else {
                            let _ = self.coverage_status_sender.send(CoverageStatus::Disabled);
                        }
                    },
            ChangeEvent::SingleTest { id, update_snapshots } => self.run_test(id, update_snapshots, cancellation.clone()),
        }
        
        self.log.info("Done sending it!");
    }
}