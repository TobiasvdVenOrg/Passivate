use std::{io::{BufRead, BufReader}, sync::mpsc::Sender};
use crate::test_execution::{RunTests, RunTestsError, SingleTest, TestRunCommand, TestsStatus};

pub struct Nextest {
    test_run_command: TestRunCommand
}

impl Nextest {
    pub fn new(test_run_command: TestRunCommand) -> Self {
        Self { test_run_command }
    }
}

impl RunTests for Nextest {
    fn run_tests(&mut self, sender: &Sender<TestsStatus>) -> Result<(), RunTestsError> {
        let _ = sender.send(TestsStatus::Running);

        let output = self.test_run_command.spawn()?;

        let mut tests: Vec<SingleTest> = vec!();

        if let Some(out) = output.stderr {
            let reader = BufReader::new(out);

            for line in reader.lines().map_while(Result::ok) {
                let test = self.test_run_command.parser.parse_line(&line);

                if let Some(test) = test {
                    tests.push(test);

                    let new_status = TestsStatus::completed(tests.clone());
                    let _ = sender.send(new_status);
                }
            }
        };

        let new_status = TestsStatus::completed(tests.clone());
        let _ = sender.send(new_status);

        Ok(())
    }
}
