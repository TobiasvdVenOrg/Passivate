use std::ffi::OsStr;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::test_execution::{SingleTest, SingleTestStatus, TestsStatus};

pub struct TestRunner {
    path: PathBuf,
    tests_status_handler: Sender<TestsStatus>
}

impl TestRunner {
    pub fn new(path: &Path, tests_status_handler: Sender<TestsStatus>) -> Self {
        TestRunner { path: path.to_path_buf(), tests_status_handler }
    }

    fn parse_status(&mut self, text: &str) -> TestsStatus {
        let mut tests = Vec::new();

        for line in text.lines() {
            println!("{}", line);

            if line.contains("error") {
                return TestsStatus::build_failure(line)
            }

            if let Some((test, result)) = split_and_trim(&line) {
                let status = match result.as_str() {
                    "ok" => SingleTestStatus::Passed,
                    _ => SingleTestStatus::Failed
                };

                let path = Path::new(OsStr::new(""));
                tests.push(SingleTest::new(test.to_string(), status, path, 0));
            }
        }

        TestsStatus::completed(tests)
    }
}

impl HandleChangeEvent for TestRunner {
    fn handle_event(&mut self, _event: ChangeEvent) {
        println!("Running...");
        let _ = self.tests_status_handler.send(TestsStatus::running());

        let output = Command::new("cargo")
            .arg("test")
            .current_dir(&self.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output().expect("Failed to execute cargo test");

        let text;

        if !output.stdout.is_empty() {
            text = String::from_utf8(output.stdout).unwrap();
        } else {
            text = String::from_utf8(output.stderr).unwrap();
        }

        let status = self.parse_status(&text);
        let _ = self.tests_status_handler.send(status);
        println!("Done...");
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