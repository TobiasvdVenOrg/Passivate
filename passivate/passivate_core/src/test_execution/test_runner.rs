use std::ffi::OsStr;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use crate::change_events::{ChangeEvent, HandleChangeEvent};
use crate::test_execution::{SingleTest, SingleTestStatus, TestsStatus};
use std::fs;

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
        println!("Path: {}", self.path.display());

        let _ = self.tests_status_handler.send(TestsStatus::running());

        let passivate_path = self.path.join(".passivate");
        let coverage_path = passivate_path.join("coverage");

        println!("Removing {}", coverage_path.display());

        let remove_result = fs::remove_dir_all(&coverage_path);

        if let Err(error) = remove_result {
            if error.kind() != ErrorKind::NotFound {
                println!("Failed to clean up .profraw files.");
            }
        }
        fs::create_dir_all(&coverage_path).unwrap(); 

        let profraw_path = fs::canonicalize(
            coverage_path).unwrap().join("coverage-%p-%m.profraw");

        println!("Profraw: {}", profraw_path.display());

        let output = Command::new("cargo")
            .current_dir(&self.path)
            .arg("test")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", profraw_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output().expect("Failed to execute cargo test");

        let text = if !output.stdout.is_empty() {
            String::from_utf8(output.stdout).unwrap()
        } else {
            String::from_utf8(output.stderr).unwrap()
        };

        let _grcov = Command::new("grcov")
            .current_dir(&self.path)
            .arg("./.passivate/coverage/")
            .arg("-s")
            .arg(".")
            .arg("--binary-path")
            .arg("./target/x86_64-pc-windows-msvc/debug/")
            .arg("-t")
            .arg("lcov")
            .arg("--branch")
            .arg("--ignore-not-existing")
            .arg("-o")
            .arg("./.passivate/coverage/lcov.info")
            .spawn()
            .unwrap()
            .wait();

        let mut coverage_path = self.path.clone();
        coverage_path.push(".passivate/coverage/lcov");
    
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