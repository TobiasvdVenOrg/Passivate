use std::{path::{Path, PathBuf}, process::{Command, Stdio}, sync::mpsc::Sender};
use crate::test_execution::{RunTests, RunTestsError, TestsStatus};

use super::parse_status;

pub struct CargoTest {
    workspace_path: PathBuf,
    build_output_path: PathBuf,
    coverage_output_path: PathBuf
}

impl CargoTest {
    pub fn new(workspace_path: &Path, build_output_path: PathBuf, profraw_output_path: &Path) -> Self {
        let coverage_output_path = profraw_output_path.join("coverage-%p-%m.profraw").to_path_buf();
        Self { workspace_path: workspace_path.to_path_buf(), build_output_path, coverage_output_path }
    }
}

impl RunTests for CargoTest {
    fn run_tests(&self, sender: &Sender<TestsStatus>) -> Result<(), RunTestsError> {
        let _ = sender.send(TestsStatus::Running);
        
        let output = Command::new("cargo")
            .current_dir(&self.workspace_path)
            .arg("test")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .arg("--target-dir")
            .arg(&self.build_output_path)
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", &self.coverage_output_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let text = if !output.stdout.is_empty() {
            String::from_utf8(output.stdout)?
        } else {
            String::from_utf8(output.stderr)?
        };

        let status = parse_status(&text);
        let _ = sender.send(status.clone());

        Ok(())
    }
}
