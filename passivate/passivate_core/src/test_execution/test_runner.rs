use std::path::PathBuf;
use std::{fs, process::Command};
use std::io::{BufRead, BufReader, Error as IoError};

use crate::configuration::TestRunnerImplementation;
use crate::cross_cutting::Log;

use super::{RunTests, TestRunError, TestRunIterator};

pub struct TestRunner {
    working_dir: PathBuf, 
    target_dir: PathBuf, 
    coverage_output_dir: PathBuf,
    log: Box<dyn Log + Send>
}

impl TestRunner {
    pub fn new(working_dir: PathBuf, target_dir: PathBuf, coverage_output_dir: PathBuf, log: Box<dyn Log + Send>) -> Self {
        Self { working_dir, target_dir, coverage_output_dir, log }
    }
}

impl RunTests for TestRunner {
    // Unable to test effectively due to non-deterministic order of cargo test output (order of tests changes)
    // During manual testing stdout and stderr output appeared to be interleaved in the correct order
    fn run_tests(&self, implementation: TestRunnerImplementation) -> Result<Box<dyn Iterator<Item = Result<String, IoError>>>, TestRunError> {

        self.log.info("Ready to run!");

        let (reader, writer) = os_pipe::pipe()?;
        let writer_clone = writer.try_clone()?;

        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = fs::canonicalize(&self.coverage_output_dir)?;

        self.log.info("Prepared output!");

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

            self.log.info("Spawned!");

        drop(process);

        self.log.info("Preparing to read!");

        // TODO: Consider rewriting without BufReader, buffering may slow down responsiveness?
        let out_reader = BufReader::new(reader);

        let stdout = out_reader.lines();

        self.log.info("Returning stdout!");

        Ok(Box::new(TestRunIterator::new(stdout)))
    }
}