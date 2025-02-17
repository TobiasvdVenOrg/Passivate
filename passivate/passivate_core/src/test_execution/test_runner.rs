use std::sync::mpsc::Sender;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::coverage::ComputeCoverage;
use crate::passivate_cargo::*;
use crate::test_execution::TestsStatus;

use super::{RunTests, RunTestsErrorStatus};

pub struct TestRunner {
    runner: Box<dyn RunTests>,
    coverage: Box<dyn ComputeCoverage>,
    tests_status_handler: Sender<TestsStatus>
}

impl TestRunner {
    pub fn new(
        runner: Box<dyn RunTests>,
        coverage: Box<dyn ComputeCoverage>, 
        tests_status_handler: Sender<TestsStatus>) -> Self {
        TestRunner {
            runner, 
            coverage, 
            tests_status_handler 
        }
    }
}

impl HandleChangeEvent for TestRunner {
    fn handle_event(&mut self, _event: ChangeEvent) {
        let _ = self.tests_status_handler.send(TestsStatus::running());

        let _ = self.coverage.clean_coverage_output();

        let test_output = self.runner.run_tests();

        match test_output {
            Ok(test_output) => {
                let _ = self.coverage.compute_coverage();
    
                let status = parse_status(&test_output);
                let _ = self.tests_status_handler.send(status);
            },
            Err(test_error) => {
                let error_status = RunTestsErrorStatus { inner_error_display: test_error.to_string() };
                let _  = self.tests_status_handler.send(TestsStatus::RunTestsError(error_status));
            }
        }
    }
}
