use std::{io::{BufRead, BufReader}, sync::mpsc::Sender};
use crate::{test_execution::{RunTests, TestRunCommand}, test_run_model::{ActiveTestRun, TestRun}};
use std::io::Error as IoError;

pub struct TestRunner {
    test_run_command: TestRunCommand
}

impl TestRunner {
    pub fn new(test_run_command: TestRunCommand) -> Self {
        Self { test_run_command }
    }
}

impl RunTests for TestRunner {
    fn run_tests(&mut self, sender: &Sender<TestRun>) -> Result<(), IoError> {
        let mut test_run = ActiveTestRun { tests: vec![] };

        let _ = sender.send(TestRun::Active(test_run.clone()));

        let output = self.test_run_command.spawn()?;

        if let Some(out) = output.stderr {
            let reader = BufReader::new(out);

            for line in reader.lines().map_while(Result::ok) {
                let test = self.test_run_command.parser.parse_line(&line);

                if let Some(test) = test {
                    test_run.tests.push(test);

                    let new_status = TestRun::Active(test_run.clone());
                    let _ = sender.send(new_status);
                }
            }
        };

        let new_status = TestRun::Active(test_run.clone());
        let _ = sender.send(new_status);

        Ok(())
    }
}
