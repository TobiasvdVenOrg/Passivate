use std::{ffi::OsString, rc::Rc};
use std::path::PathBuf;
use std::fs;
use std::io::Error as IoError;

use duct::cmd;

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
    fn run_tests(&self, implementation: TestRunnerImplementation, instrument_coverage: bool) -> Result<Box<dyn Iterator<Item = Result<Rc<String>, IoError>>>, TestRunError> {
        self.log.info("Ready to run!");

        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = fs::canonicalize(&self.coverage_output_dir)?;

        self.log.info("Prepared output!");

        let mut args: Vec<OsString> = vec![];
        match implementation {
            TestRunnerImplementation::Cargo => args.push(OsString::from("test")),
            TestRunnerImplementation::Nextest => {
                args.push(OsString::from("nextest"));
                args.push(OsString::from("run"));
            }
        };

        args.push(OsString::from("--no-fail-fast"));
        args.push(OsString::from("--target"));
        args.push(OsString::from("x86_64-pc-windows-msvc"));
        args.push(OsString::from("--target-dir"));
        args.push(OsString::from(&self.target_dir));

        let command = cmd("cargo", args).dir(&self.working_dir);

        let command = if instrument_coverage {
            command
                .env("RUSTFLAGS", "-C instrument-coverage")
                .env("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"))
        } else {
            command
        };

        let stdout = command.stderr_to_stdout().reader()?;

        Ok(Box::new(TestRunIterator::new(stdout)))
    }
}