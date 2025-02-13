use std::{path::Path, process::{Command, Stdio}};

pub fn cargo_test(working_dir: &Path, profraw_output_path: &Path) -> String {
    let output = Command::new("cargo")
            .current_dir(working_dir)
            .arg("test")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", profraw_output_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output().expect("Failed to execute cargo test");

        let text = if !output.stdout.is_empty() {
            String::from_utf8(output.stdout).unwrap()
        } else {
            String::from_utf8(output.stderr).unwrap()
        };

        text
}