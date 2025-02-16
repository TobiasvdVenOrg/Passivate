use std::{fs, path::Path, process::{Command, Stdio}};
use crate::test_execution::{RunTests, RunTestsError};

pub struct CargoTest {

}

impl CargoTest {

}

impl RunTests for CargoTest {
    fn run_tests(&self, path: &Path, profraw_output_dir: &Path) -> Result<String, RunTestsError> {
        // Absolute dir, because a relative dir will cause profraw files to be output relative to each individual project in the workspace
        let absolute_profraw_output_dir = fs::canonicalize(profraw_output_dir)?.join("coverage-%p-%m.profraw");
        
        let output = Command::new("cargo")
            .current_dir(path)
            .arg("test")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", absolute_profraw_output_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let text = if !output.stdout.is_empty() {
            String::from_utf8(output.stdout)?
        } else {
            String::from_utf8(output.stderr)?
        };

        Ok(text)
    }
}
