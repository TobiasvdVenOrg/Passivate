use std::path::PathBuf;
use std::{fs, process::Command};
use std::io::{BufRead, BufReader, Error as IoError};

use crate::configuration::TestRunnerImplementation;

use super::{RunTests, TestRunIterator};

pub struct TestRunner {
    working_dir: PathBuf, 
    target_dir: PathBuf, 
    coverage_output_dir: PathBuf
}

impl TestRunner {
    pub fn new(working_dir: PathBuf, target_dir: PathBuf, coverage_output_dir: PathBuf) -> Self {
        Self { working_dir, target_dir, coverage_output_dir }
    }
}

impl RunTests for TestRunner {
    // Unable to test effectively due to non-deterministic order of cargo test output (order of tests changes)
    // During manual testing stdout and stderr output appeared to be interleaved in the correct order
    fn run_tests(&self, implementation: TestRunnerImplementation) -> Result<Box<dyn Iterator<Item = Result<String, IoError>>>, IoError> {
        let (reader, writer) = os_pipe::pipe()?;
        let writer_clone = writer.try_clone()?;

        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = fs::canonicalize(&self.coverage_output_dir)?;

        let mut command = Command::new("cargo");
        let command = command.current_dir(&self.working_dir);

        let command = match implementation {
            TestRunnerImplementation::Cargo => command.arg("test"),
            TestRunnerImplementation::Nextest => command.arg("nextest").arg("run")
        };

        let process = command
            .arg("--no-fail-fast")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .arg("--target-dir")
            .arg(&self.target_dir)
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"))    
            .stdout(writer)
            .stderr(writer_clone)
            .spawn()?;
        
        drop(process);

        // TODO: Consider rewriting without BufReader, buffering may slow down responsiveness?
        let out_reader = BufReader::new(reader);

        let stdout = out_reader.lines();

        Ok(Box::new(TestRunIterator::new(stdout)))
    }
}