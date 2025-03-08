use std::sync::mpsc::Sender;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::coverage::{ComputeCoverage, CoverageStatus};
use crate::test_execution::TestRun;
use super::{FailedTestRun, RunTests};

pub struct TestRunner {
    runner: Box<dyn RunTests>,
    coverage: Box<dyn ComputeCoverage>,
    tests_status_sender: Sender<TestRun>,
    coverage_status_sender: Sender<CoverageStatus>
}

impl TestRunner {
    pub fn new(
        runner: Box<dyn RunTests>,
        coverage: Box<dyn ComputeCoverage>, 
        tests_status_sender: Sender<TestRun>,
        coverage_status_sender: Sender<CoverageStatus>) -> Self {
            Self {
            runner, 
            coverage, 
            tests_status_sender,
            coverage_status_sender
        }
    }
}

impl HandleChangeEvent for TestRunner {
    fn handle_event(&mut self, _event: ChangeEvent) {
        self.coverage.clean_coverage_output().unwrap();

        let test_output = self.runner.run_tests(&self.tests_status_sender);

        match test_output {
            Ok(_) => {
                let coverage_status = self.coverage.compute_coverage();

                let _ = match coverage_status {
                    Ok(coverage_status) => self.coverage_status_sender.send(coverage_status),
                    Err(coverage_error) => self.coverage_status_sender.send(CoverageStatus::Error(coverage_error))
                };
            },
            Err(test_error) => {
                let error_status = FailedTestRun { inner_error_display: test_error.to_string() };
                let _  = self.tests_status_sender.send(TestRun::Failed(error_status));
            }
        };
    }
}
