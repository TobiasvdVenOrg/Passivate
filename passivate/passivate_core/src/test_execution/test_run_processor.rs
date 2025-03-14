use std::sync::mpsc::Sender;
use crate::test_run_model::{ActiveTestRun, TestRun, TestRunEvent};
use std::io::Error as IoError;

use super::{ParseOutput, RunTests};

pub struct TestRunProcessor {
    run_tests: Box<dyn RunTests>,
    parse_output: Box<dyn ParseOutput>,
    active_test_run: ActiveTestRun
}

impl TestRunProcessor {
    pub fn new(run_tests: Box<dyn RunTests>, parse_output: Box<dyn ParseOutput>) -> Self {
        Self { run_tests, parse_output, active_test_run: ActiveTestRun::default() }
    }

    fn update(&mut self, event: TestRunEvent, sender: &Sender<TestRun>) {
        if self.active_test_run.update(event) {
            let _ = sender.send(TestRun::Active(self.active_test_run.clone()));
        }
    }

    pub fn run_tests(&mut self, sender: &Sender<TestRun>) -> Result<(), IoError> {
        self.update(TestRunEvent::Start, sender);

        let iterator = self.run_tests.run_tests(self.parse_output.get_implementation())?;

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
