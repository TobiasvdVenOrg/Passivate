use std::{io::{BufRead, BufReader}, path::{Path, PathBuf}, process::{Command, Stdio}, sync::mpsc::Sender};
use crate::test_execution::{RunTests, RunTestsError, SingleTest, TestsStatus};

use super::parse_line;

pub struct Nextest {
    workspace_path: PathBuf,
    coverage_output_path: PathBuf
}

impl Nextest {
    pub fn new(workspace_path: &Path, profraw_output_path: &Path) -> Self {
        let coverage_output_path = profraw_output_path.join("coverage-%p-%m.profraw").to_path_buf();
        Self { workspace_path: workspace_path.to_path_buf(), coverage_output_path }
    }
}

impl RunTests for Nextest {
    fn run_tests(&self, sender: &Sender<TestsStatus>) -> Result<(), RunTestsError> {
        let _ = sender.send(TestsStatus::Running);
        
        let output = Command::new("cargo")
            .current_dir(&self.workspace_path)
            .arg("nextest")
            .arg("run")
            .arg("--no-fail-fast")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", &self.coverage_output_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(out) = output.stdout {
            let reader = BufReader::new(out);
            let mut tests: Vec<SingleTest> = vec!();

            for line in reader.lines().map_while(Result::ok) {
                let test = parse_line(&line);

                if let Some(test) = test {
                    tests.push(test);

                    let new_status = TestsStatus::completed(tests.clone());
                    let _ = sender.send(new_status);
                }
            }
        };

        if let Some(out) = output.stderr {
            let reader = BufReader::new(out);
            let mut tests: Vec<SingleTest> = vec!();

            for line in reader.lines().map_while(Result::ok) {
                let test = parse_line(&line);

                if let Some(test) = test {
                    tests.push(test);

                    let new_status = TestsStatus::completed(tests.clone());
                    let _ = sender.send(new_status);
                }
            }
        };

        Ok(())
    }
}
