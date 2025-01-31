use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::change_events::{ChangeEvent, ChangeEventHandler};
use crate::tests_view::{SingleTest, SingleTestStatus, TestsStatus, TestsStatusHandler};

pub struct TestExecution {
    tests_status_handler: Box<dyn TestsStatusHandler>
}

impl TestExecution {
    pub fn new(tests_status_handler: Box<dyn TestsStatusHandler>) -> Self {
        TestExecution { tests_status_handler }
    }
}

impl ChangeEventHandler for TestExecution {
    fn handle_event(&mut self, _event: ChangeEvent) {
        self.tests_status_handler.refresh(TestsStatus { running: true, tests: Vec::new() });

        let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");
        let output = Command::new("cargo").arg("test").current_dir(path).stdout(Stdio::piped()).spawn().expect("Failed to run tests.");

        let stdout = output.stdout.expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        let mut tests = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line");

            if let Some((test, result)) = split_and_trim(&line) {
                let status = match result.as_str() {
                    "ok" => SingleTestStatus::Passed,
                    _ => SingleTestStatus::Failed
                };

                tests.push(SingleTest { name: test.to_string(), status });
            }
        }

        self.tests_status_handler.refresh(TestsStatus { running: false, tests });
    }
}

fn split_and_trim(line: &str) -> Option<(String, String)> {
    // Split the line into at most two parts by "..."
    let mut parts = line.splitn(2, "...");

    // Get the first and second parts, if they exist
    let first = parts.next()?.trim().to_string();  // Get and trim first part
    let second = parts.next()?.trim().to_string(); // Get and trim second part

    Some((first, second))
}