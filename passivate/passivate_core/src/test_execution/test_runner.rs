use std::{path::PathBuf, sync::mpsc::Sender};
use crate::{test_execution::RunTests, test_run_model::{ActiveTestRun, TestRun, TestRunEvent}};
use std::io::Error as IoError;

use super::{ParseOutput, TestRunIterator};

pub struct TestRunner {
    parse_output: Box<dyn ParseOutput>,
    working_dir: PathBuf, 
    target_dir: PathBuf, 
    coverage_output_dir: PathBuf,
    active_test_run: ActiveTestRun
}

impl TestRunner {
    pub fn new(parse_output: Box<dyn ParseOutput>, working_dir: PathBuf, target_dir: PathBuf, coverage_output_dir: PathBuf) -> Self {
        Self { parse_output, working_dir, target_dir, coverage_output_dir, active_test_run: ActiveTestRun::default() }
    }

    fn update(&mut self, event: TestRunEvent, sender: &Sender<TestRun>) {
        if self.active_test_run.update(event) {
            let _ = sender.send(TestRun::Active(self.active_test_run.clone()));
        }
    }
}

impl RunTests for TestRunner {
    fn run_tests(&mut self, sender: &Sender<TestRun>) -> Result<(), IoError> {
        self.update(TestRunEvent::Start, sender);

        let iterator = TestRunIterator::run_tests(
            self.parse_output.get_implementation(), 
            &self.working_dir, 
            &self.target_dir, 
            &self.coverage_output_dir)?;

        for line in iterator {
            let test_run_event = self.parse_output.parse_line(&line.unwrap());

            if let Some(test_run_event) = test_run_event {
                self.update(test_run_event, sender);
            }
        }

        if self.active_test_run.tests.is_empty() {
            self.update(TestRunEvent::NoTests, sender);
        }

        Ok(())
    }
}
