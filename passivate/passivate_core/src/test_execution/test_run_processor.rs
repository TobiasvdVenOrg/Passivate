use std::sync::mpsc::Sender;
use crate::{actors::Cancellation, cross_cutting::Log, test_run_model::{TestRun, TestRunEvent}};

use super::{ParseOutput, RunTests, TestRunError};

pub struct TestRunProcessor {
    run_tests: Box<dyn RunTests + Send>,
    parse_output: Box<dyn ParseOutput + Send>,
    test_run: TestRun,
    log: Box<dyn Log + Send>
}

impl TestRunProcessor {
    pub fn new(run_tests: Box<dyn RunTests + Send>, parse_output: Box<dyn ParseOutput + Send>, log: Box<dyn Log + Send>) -> Self {
        Self::from_test_run(run_tests, parse_output, TestRun::default(), log)
    }

    pub fn from_test_run(run_tests: Box<dyn RunTests + Send>, parse_output: Box<dyn ParseOutput + Send>, test_run: TestRun, log: Box<dyn Log + Send>) -> Self {
        Self { run_tests, parse_output, test_run, log }
    }

    fn update(&mut self, event: TestRunEvent, sender: &Sender<TestRun>) {
        if self.test_run.update(event) {
            let _ = sender.send(self.test_run.clone());
        }
    }

    pub fn run_tests(&mut self, sender: &Sender<TestRun>, instrument_coverage: bool, cancellation: Cancellation) -> Result<(), TestRunError> {
        self.update(TestRunEvent::Start, sender);

        cancellation.check()?;

        self.log.info("Running the tests...");

        let iterator = self.run_tests.run_tests(self.parse_output.get_implementation(), instrument_coverage)?;

        cancellation.check()?;

        self.log.info("Parsing the tests...");

        for line in iterator {
            let test_run_event = self.parse_output.parse_line(&line.unwrap());

            cancellation.check()?;

            if let Some(test_run_event) = test_run_event {
                self.update(test_run_event, sender);
            }

            cancellation.check()?;
        }

        self.log.info("Done with the tests...");

        if self.test_run.tests.is_empty() {
            self.update(TestRunEvent::NoTests, sender);
        }

        Ok(())
    }
}
