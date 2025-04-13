use crate::{delegation::{Cancellation, Tx}, test_run_model::{TestId, TestRun, TestRunEvent, TestRunState}};

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

    fn update(&mut self, event: TestRunEvent, sender: &Tx<TestRun>) {
        if self.test_run.update(event) {
            sender.send(self.test_run.clone());
        }
    }

    pub fn run_tests(&mut self, sender: &Tx<TestRun>, instrument_coverage: bool, cancellation: Cancellation) -> Result<(), TestRunError> {
        self.update(TestRunEvent::Start, sender);

        cancellation.check()?;

        let iterator = self.run_tests.run_tests(self.parse_output.get_implementation(), instrument_coverage, cancellation.clone())?;

        cancellation.check()?;

        for line in iterator {
            match line {
                Ok(line) => {
                    let test_run_event = self.parse_output.parse_line(&line);

                    cancellation.check()?;

                    if let Some(test_run_event) = test_run_event {
                        self.update(test_run_event, sender);
                    }
                },
                Err(_error) => {
                    break;
                },
            }

            cancellation.check()?;
        }

        match self.test_run.state {
            TestRunState::BuildFailed(_) => {
                
            },
            _ => {
                if self.test_run.tests.is_empty() {
                    self.update(TestRunEvent::NoTests, sender);
                } else {
                    self.update(TestRunEvent::TestsCompleted, sender);
                }
            }
        }     

        Ok(())
    }
    
    pub fn run_test(&mut self, sender: &Tx<TestRun>, id: &TestId, update_snapshots: bool, cancellation: Cancellation) -> Result<(), TestRunError> {
        if let Some(test) = self.test_run.tests.find(id) {
            self.update(TestRunEvent::StartSingle {
                test: id.clone(),
                clear_tests: !update_snapshots // if we're just updating a snapshot we don't need to clear the other tests
            }, sender);

            let iterator = self.run_tests.run_test(self.parse_output.get_implementation(), &test.name, update_snapshots, cancellation.clone())?;

            cancellation.check()?;

            for line in iterator {
                match line {
                    Ok(line) => {
                        let test_run_event = self.parse_output.parse_line(&line);
    
                        cancellation.check()?;
    
                        if let Some(test_run_event) = test_run_event {
                            self.update(test_run_event, sender);
                        }
                    },
                    Err(_error) => {
                        break;
                    },
                }
    
                cancellation.check()?;
            }

            self.update(TestRunEvent::TestsCompleted, sender);
        }

        Ok(())
    }
}
