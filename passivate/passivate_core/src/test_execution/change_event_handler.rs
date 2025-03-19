use std::sync::mpsc::Sender;
use crate::actors::{Cancellation, Handler};
use crate::change_events::ChangeEvent;
use crate::coverage::{ComputeCoverage, CoverageStatus};
use crate::test_run_model::{FailedTestRun, TestRun};
use super::TestRunProcessor;

pub struct ChangeEventHandler {
    runner: TestRunProcessor,
    coverage: Box<dyn ComputeCoverage + Send>,
    tests_status_sender: Sender<TestRun>,
    coverage_status_sender: Sender<CoverageStatus>,
    previous_run_cancellation: Option<Cancellation>
}

impl ChangeEventHandler {
    pub fn new(
        runner: TestRunProcessor,
        coverage: Box<dyn ComputeCoverage + Send>, 
        tests_status_sender: Sender<TestRun>,
        coverage_status_sender: Sender<CoverageStatus>) -> Self {
            Self {
            runner, 
            coverage, 
            tests_status_sender,
            coverage_status_sender,
            previous_run_cancellation: None
        }
    }
}

impl Handler<ChangeEvent> for ChangeEventHandler {
    fn handle(&mut self, _event: ChangeEvent, cancellation: Cancellation) {
        if let Some(mut previous_run) = self.previous_run_cancellation.clone() {
            previous_run.cancel();
        }

        self.previous_run_cancellation = Some(cancellation.clone());

        println!("Handling it!");
        self.coverage.clean_coverage_output().unwrap();

        println!("Done cleaning it!");
        if cancellation.is_cancelled() { return }

        let test_output = self.runner.run_tests(&self.tests_status_sender, cancellation.clone());

        println!("Done running it!");

        if cancellation.is_cancelled() { return }

        match test_output {
            Ok(_) => {
                let coverage_status = self.coverage.compute_coverage(cancellation.clone());

                println!("Done covering it!");

                let _ = match coverage_status {
                    Ok(coverage_status) => self.coverage_status_sender.send(coverage_status),
                    Err(coverage_error) => self.coverage_status_sender.send(CoverageStatus::Error(coverage_error))
                };
            },
            Err(test_error) => {
                let error_status = FailedTestRun { inner_error_display: test_error.to_string() };
                let _  = self.tests_status_sender.send(TestRun::from_failed(error_status));
            }
        };

        println!("Done sending it!");
    }
}
