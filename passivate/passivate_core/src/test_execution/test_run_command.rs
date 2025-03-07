use std::{fs, path::{Path, PathBuf}, process::{Child, Command, Stdio}};
use std::io::Error as IoError;
use crate::configuration::TestRunnerImplementation;
use crate::passivate_cargo::CargoTestParser;
use crate::passivate_nextest::NextestParser;
use super::ParseOutput;

pub struct TestRunCommand {
    command: Command,
    coverage_output_dir: Option<PathBuf>,
    pub parser: Box<dyn ParseOutput>
}

impl TestRunCommand {
    pub fn for_implementation(implementation: &TestRunnerImplementation) -> Self {
        let parser: Box<dyn ParseOutput> = match implementation {
            TestRunnerImplementation::Cargo => Box::new(CargoTestParser),
            TestRunnerImplementation::Nextest => Box::new(NextestParser),
        };

        let mut command = TestRunCommand { 
            command: Command::new("cargo"),
            coverage_output_dir: None,
            parser
        };

        match implementation {
            TestRunnerImplementation::Cargo => command.command.arg("test"),
            TestRunnerImplementation::Nextest => command.command.arg("nextest").arg("run")
        };

        command.command
            .arg("--no-fail-fast")
            .arg("--target")
            .arg("x86_64-pc-windows-msvc")         
            .env("RUSTFLAGS", "-C instrument-coverage")            
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        command
    }

    pub fn working_dir(mut self, working_dir: &Path) -> Self {
        self.command.current_dir(working_dir);
        self
    }

    pub fn target_dir(mut self, target_dir: &Path) -> Self {
        self.command
            .arg("--target-dir")
            .arg(target_dir);

        self
    }

    pub fn coverage_output_dir(mut self, coverage_output_dir: &Path) -> Self {
        self.coverage_output_dir = Some(coverage_output_dir.to_path_buf());    
        self
    }

    pub fn spawn(&mut self) -> Result<Child, IoError> {
        if let Some(coverage_output_dir) = &self.coverage_output_dir {
            fs::create_dir_all(coverage_output_dir)?;
            let coverage_output_dir = fs::canonicalize(coverage_output_dir)?;
            self.command.env("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"));
        }

        self.command.spawn()
    }
}
