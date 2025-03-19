use std::sync::mpsc::Sender;
use crate::{actors::Cancellation, test_run_model::{TestRun, TestRunEvent}};

use super::{ParseOutput, RunTests, TestRunError};

pub struct TestRunProcessor {
    run_tests: Box<dyn RunTests + Send>,
    parse_output: Box<dyn ParseOutput + Send>,
    test_run: TestRun
}

impl TestRunProcessor {
    pub fn new(run_tests: Box<dyn RunTests + Send>, parse_output: Box<dyn ParseOutput + Send>) -> Self {
        Self::from_test_run(run_tests, parse_output, TestRun::default())
    }

    pub fn from_test_run(run_tests: Box<dyn RunTests + Send>, parse_output: Box<dyn ParseOutput + Send>, test_run: TestRun) -> Self {
        Self { run_tests, parse_output, test_run }
    }

    fn update(&mut self, event: TestRunEvent, sender: &Sender<TestRun>) {
        if self.test_run.update(event) {
            let _ = sender.send(self.test_run.clone());
        }
    }

    pub fn run_tests(&mut self, sender: &Sender<TestRun>, cancellation: Cancellation) -> Result<(), TestRunError> {
        self.update(TestRunEvent::Start, sender);

        cancellation.check()?;

        let iterator = self.run_tests.run_tests(self.parse_output.get_implementation())?;

        cancellation.check()?;

        for line in iterator {
            let test_run_event = self.parse_output.parse_line(&line.unwrap());

            cancellation.check()?;

            if let Some(test_run_event) = test_run_event {
                self.update(test_run_event, sender);
            }

            cancellation.check()?;
        }

        if self.test_run.tests.is_empty() {
            self.update(TestRunEvent::NoTests, sender);
        }

        Ok(())
    }
}
