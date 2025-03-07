use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::io::Error as IoError;
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::configuration::TestRunner;
use passivate_core::coverage::CoverageStatus;
use passivate_core::passivate_cargo::CargoTest;
use passivate_core::passivate_nextest::Nextest;
use passivate_core::test_execution::RunTests;
use passivate_core::{passivate_grcov::Grcov, test_execution::TestsStatus};

pub struct TestRunnerBuilder {
    test_runner: TestRunner,
    tests_status_sender: Option<Sender<TestsStatus>>,
    coverage_sender: Option<Sender<CoverageStatus>>,
    base_workspace_path: PathBuf,
    base_output_path: PathBuf,
    workspace_path: PathBuf,
    output_path: PathBuf
}

pub fn test_data_path() -> PathBuf {
    fs::canonicalize(PathBuf::from("../../test_data")).expect("Test data path did not exist!")
}

pub fn test_output_path() -> PathBuf {
    fs::canonicalize(PathBuf::from("../../test_output")).expect("Test output path did not exist!")
}

pub fn cargo_builder() -> TestRunnerBuilder {
    TestRunnerBuilder::cargo(test_data_path(), test_output_path())
}

pub fn nextest_builder() -> TestRunnerBuilder {
    TestRunnerBuilder::nextest(test_data_path(), test_output_path())
}

impl TestRunnerBuilder {
    pub fn cargo(base_workspace_path: PathBuf, base_output_path: PathBuf) -> Self {
        Self::new(TestRunner::Cargo, base_workspace_path, base_output_path)
    }

    pub fn nextest(base_workspace_path: PathBuf, base_output_path: PathBuf) -> Self {
        Self::new(TestRunner::Nextest, base_workspace_path, base_output_path)
    }

    pub fn new(test_runner: TestRunner, base_workspace_path: PathBuf, base_output_path: PathBuf) -> Self {
        Self { 
            test_runner, 
            tests_status_sender: None, 
            coverage_sender: None, 
            base_workspace_path, 
            base_output_path,
            workspace_path: PathBuf::from(""),
            output_path: PathBuf::from("")
        }
    }

    pub fn receive_tests_status(&mut self, sender: Sender<TestsStatus>) -> &mut Self {
        self.tests_status_sender = Some(sender);
        self
    }

    pub fn receive_coverage_status(&mut self, sender: Sender<CoverageStatus>) -> &mut Self {
        self.coverage_sender = Some(sender);
        self
    }

    pub fn with_workspace(&mut self, workspace_path: &str) -> &mut Self {
        self.workspace_path.push(workspace_path);
        self
    }

    pub fn with_output(&mut self, output_path: &str) -> &mut Self {
        self.output_path.push(output_path);
        self
    }

    pub fn build(&self) -> Result<passivate_core::test_execution::TestRunner, IoError> {
        let workspace_path = self.get_workspace_path();
        let output_path = self.get_output_path();

        let passivate_path = output_path.join(".passivate");
        let binary_path = output_path.join("x86_64-pc-windows-msvc/debug");
        let coverage_path = passivate_path.join("coverage");

        // FIXME?
        if fs::exists(&output_path)? {
            fs::remove_dir_all(&output_path)?
        }

        // FIXME
        fs::create_dir_all(&coverage_path)?;

        // FIXME
        let absolute_coverage_path = fs::canonicalize(&coverage_path)?; 

        let grcov = Grcov::new(&workspace_path, &coverage_path, &binary_path);

        let runner: Box<dyn RunTests> = match self.test_runner {
            TestRunner::Cargo => Box::new(CargoTest::new(&workspace_path, output_path, &absolute_coverage_path)),
            TestRunner::Nextest => Box::new(Nextest::new(&workspace_path, output_path, &absolute_coverage_path)),
        };

        let tests_status_sender = self.tests_status_sender.clone().unwrap_or(channel().0);
        let coverage_sender = self.coverage_sender.clone().unwrap_or(channel().0);

        Ok(passivate_core::test_execution::TestRunner::new(
            runner, 
            Box::new(grcov), 
            tests_status_sender, 
            coverage_sender))
    }

    pub fn get_workspace_path(&self) -> PathBuf {
        self.base_workspace_path.join(&self.workspace_path)
    }

    pub fn get_output_path(&self) -> PathBuf {
        self.base_output_path.join(&self.output_path).join(self.runner_identifier())
    }

    fn runner_identifier(&self) -> PathBuf {
        match self.test_runner {
            TestRunner::Cargo => PathBuf::from("cargo"),
            TestRunner::Nextest => PathBuf::from("nextest")
        }
    }
}

pub fn test_run(test_runner: &mut passivate_core::test_execution::TestRunner) -> Result<(), IoError> {

    let mock_event = ChangeEvent::File;
    test_runner.handle_event(mock_event);

    Ok(())
}

// TODO:
// Add way to add arbitrary environment vars
// So we can add CARGO_TARGET_DIR = test_output/test_name
// Add optional override for .passivate directory (defaults to root of workspace, but can be set otherwise)
// Pass in 2 paths per test here, test_data_path and test_output_path