use std::{io::{BufRead, BufReader}, sync::mpsc::Sender};
use crate::{test_execution::{RunTests, TestRunCommand}, test_run_model::{ActiveTestRun, TestRun, TestRunEvent}};
use std::io::Error as IoError;

pub struct TestRunner {
    test_run_command: TestRunCommand,
    active_test_run: ActiveTestRun
}

impl TestRunner {
    pub fn new(test_run_command: TestRunCommand) -> Self {
        Self { test_run_command, active_test_run: ActiveTestRun { tests: vec![] } }
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

        let output = self.test_run_command.spawn()?;

        if let (Some(std_out), Some(std_err)) = (output.stdout, output.stderr) {
            let out_reader = BufReader::new(std_out);
            let err_reader = BufReader::new(std_err);

            let mut out_lines = out_reader.lines();
            let mut err_lines = err_reader.lines();

            loop {
                let out_next = out_lines.next();
                let err_next = err_lines.next();

                let mut exit = 0;

                if let Some(out_next) = out_next {
                    let out_line = out_next.unwrap();

                    let test = self.test_run_command.parser.parse_line(&out_line);

                    if let Some(test) = test {
                        self.update(test, sender);
                    }
                } else {
                    exit += 1;
                }

                if let Some(err_next) = err_next {
                    let err_line = err_next.unwrap();

                    let test = self.test_run_command.parser.parse_line(&err_line);

                    if let Some(test) = test {
                        self.update(test, sender);
                    }
                } else {
                    exit += 1;
                }

                if exit > 1 {
                    break;
                }
            }
        };

        if self.active_test_run.tests.is_empty() {
            self.update(TestRunEvent::NoTests, sender);
        }

        Ok(())
    }
}
