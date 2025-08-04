use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use duct::cmd;
use passivate_delegation::Cancellation;

use super::{RunTests, TestRunError, TestRunIterator};
use crate::configuration::TestRunnerImplementation;

pub struct TestRunner
{
    target: OsString,
    working_dir: PathBuf,
    target_dir: PathBuf,
    coverage_output_dir: PathBuf
}

impl TestRunner
{
    pub fn new(target: OsString, working_dir: PathBuf, target_dir: PathBuf, coverage_output_dir: PathBuf) -> Self
    {
        Self {
            target,
            working_dir,
            target_dir,
            coverage_output_dir
        }
    }
}

impl RunTests for TestRunner
{
    // Unable to test effectively due to non-deterministic order of cargo test output (order of tests changes)
    // During manual testing stdout and stderr output appeared to be interleaved in the correct order
    fn run_tests(
        &self,
        implementation: TestRunnerImplementation,
        instrument_coverage: bool,
        cancellation: Cancellation
    ) -> Result<Box<dyn Iterator<Item = Result<Rc<String>, TestRunError>>>, TestRunError>
    {
        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = fs::canonicalize(&self.coverage_output_dir)?;

        let mut args: Vec<OsString> = vec![];
        match implementation
        {
            TestRunnerImplementation::Cargo => args.push(OsString::from("test")),
            TestRunnerImplementation::Nextest =>
            {
                args.push(OsString::from("nextest"));
                args.push(OsString::from("run"));
            }
        };

        args.push(OsString::from("--no-fail-fast"));
        args.push(OsString::from("--target"));
        args.push(self.target.clone());
        args.push(OsString::from("--target-dir"));
        args.push(OsString::from(&self.target_dir));

        let command = cmd("cargo", args).dir(&self.working_dir).env("RUST_BACKTRACE", "0");

        let command = if instrument_coverage
        {
            command
                .env("RUSTFLAGS", "-C instrument-coverage")
                .env("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"))
        }
        else
        {
            command
        };

        let stdout = command.stderr_to_stdout().reader()?;

        Ok(Box::new(TestRunIterator::new(stdout, cancellation)))
    }

    fn run_test(
        &self,
        implementation: TestRunnerImplementation,
        test_name: &str,
        update_snapshots: bool,
        cancellation: Cancellation
    ) -> Result<Box<dyn Iterator<Item = Result<Rc<String>, TestRunError>>>, TestRunError>
    {
        let mut args: Vec<OsString> = vec![];
        match implementation
        {
            TestRunnerImplementation::Cargo => args.push(OsString::from("test")),
            TestRunnerImplementation::Nextest =>
            {
                args.push(OsString::from("nextest"));
                args.push(OsString::from("run"));
            }
        };

        args.push(OsString::from(&test_name));
        args.push(OsString::from("--no-fail-fast"));
        args.push(OsString::from("--target"));
        args.push(OsString::from("x86_64-pc-windows-msvc"));
        args.push(OsString::from("--target-dir"));
        args.push(OsString::from(&self.target_dir));

        if update_snapshots
        {
            args.push(OsString::from("--all-features"));
        }
        
        let command = cmd("cargo", args).dir(&self.working_dir);

        let command = if update_snapshots { command.env("UPDATE_SNAPSHOTS", "1") } else { command };

        let stdout = command.stderr_to_stdout().reader()?;

        Ok(Box::new(TestRunIterator::new(stdout, cancellation)))
    }
}
