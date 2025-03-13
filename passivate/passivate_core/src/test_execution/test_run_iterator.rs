use std::{fs, io::{BufRead, BufReader, Lines}, path::Path, process::Command};
use std::io::Error as IoError;
use crate::configuration::TestRunnerImplementation;

pub struct TestRunIterator {
    stdout: Lines<BufReader<os_pipe::PipeReader>>
}

impl TestRunIterator {
    // Unable to test effectively due to non-deterministic order of cargo test output (order of tests changes)
    // During manual testing stdout and stderr output appeared to be interleaved in the correct order
    pub fn run_tests(implementation: TestRunnerImplementation, working_dir: &Path, target_dir: &Path, coverage_output_dir: &Path) -> Result<TestRunIterator, IoError> {
        let (reader, writer) = os_pipe::pipe()?;
        let writer_clone = writer.try_clone()?;

        fs::create_dir_all(coverage_output_dir)?;
        let coverage_output_dir = fs::canonicalize(coverage_output_dir)?;

        let implementation_arg = match implementation {
            TestRunnerImplementation::Cargo => "test",
            TestRunnerImplementation::Nextest => "nextest run"
        };

        let mut command = Command::new("cargo");

        match implementation {
            TestRunnerImplementation::Cargo => command.arg("test"),
            TestRunnerImplementation::Nextest => command.arg("nextest").arg("run")
        };

        command
            .current_dir(working_dir)
            .arg(implementation_arg)
            .arg("--no-fail-fast")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")
            .arg("--target-dir")
            .arg(target_dir)
            .env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"))    
            .stdout(writer)
            .stderr(writer_clone)
            .spawn()?;
        
        drop(command);

        // TODO: Consider rewriting without BufReader, buffering may slow down responsiveness?
        let out_reader = BufReader::new(reader);

        let stdout = out_reader.lines();

        Ok(TestRunIterator { stdout })
    }
}

impl Iterator for TestRunIterator {
    type Item = Result<String, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
            self.stdout.next()
    }
}