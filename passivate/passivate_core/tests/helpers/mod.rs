// Because Rust treats each .rs file in the /tests directory as its own crate for some ungodly reason,
// functions in here can be shown by the analyzer as "unused", though they may be used in some other
// .rs file than the one its currently decided to show analysis for
#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::io::Error as IoError;
use passivate_core::actors::Cancellation;
use passivate_core::actors::Handler;
use passivate_core::change_events::ChangeEvent;
use passivate_core::configuration::TestRunnerImplementation;
use passivate_core::coverage::CoverageStatus;
use passivate_core::cross_cutting::stub_log;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::build_test_output_parser;
use passivate_core::test_execution::ParseOutput;
use passivate_core::test_execution::ChangeEventHandler;
use passivate_core::test_execution::TestRunProcessor;
use passivate_core::test_execution::TestRunner;
use passivate_core::test_run_model::TestRun;

pub struct TestRunnerBuilder {
    test_runner: TestRunnerImplementation,
    tests_status_sender: Option<Sender<TestRun>>,
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
        Self::new(TestRunnerImplementation::Cargo, base_workspace_path, base_output_path)
    }

    pub fn nextest(base_workspace_path: PathBuf, base_output_path: PathBuf) -> Self {
        Self::new(TestRunnerImplementation::Nextest, base_workspace_path, base_output_path)
    }

    pub fn new(test_runner: TestRunnerImplementation, base_workspace_path: PathBuf, base_output_path: PathBuf) -> Self {
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

    pub fn receive_tests_status(&mut self, sender: Sender<TestRun>) -> &mut Self {
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
        let output_path = output_path.split("::").last().expect("Failed!");
        self.output_path.push(output_path);
        self
    }

    pub fn build_grcov(&self) -> Grcov {
        Grcov::new(&self.get_workspace_path(), &self.get_coverage_path(), &self.get_binary_path())
    }

    pub fn build(&self) -> ChangeEventHandler {
        let parser: Box<dyn ParseOutput + Send> = build_test_output_parser(&self.test_runner);
        let runner = Box::new(TestRunner::new(
            self.get_workspace_path().clone(), 
            self.get_output_path().clone(), 
            self.get_coverage_path().clone(),
            stub_log()
        ));

        let processor = TestRunProcessor::new(runner, parser, stub_log());

        let tests_status_sender = self.tests_status_sender.clone().unwrap_or(channel().0);
        let coverage_sender = self.coverage_sender.clone().unwrap_or(channel().0);

        let grcov = self.build_grcov();
        let coverage_enabled = true;

        ChangeEventHandler::new(
            processor, 
            Box::new(grcov), 
            tests_status_sender, 
            coverage_sender,
            stub_log(),
            coverage_enabled)
    }

    pub fn clean_output(&mut self) -> &mut Self {
        let output_path = self.get_output_path();

        if fs::exists(&output_path).expect("Failed to check if output_path exists!") {
            fs::remove_dir_all(&output_path).expect("Failed to clear output path!")
        }

        self
    }

    pub fn get_workspace_path(&self) -> PathBuf {
        self.base_workspace_path.join(&self.workspace_path)
    }

    pub fn get_output_path(&self) -> PathBuf {
        self.base_output_path.join(&self.output_path).join(self.runner_identifier())
    }

    pub fn get_passivate_path(&self) -> PathBuf {
        self.get_output_path().join(".passivate")
    }

    pub fn get_coverage_path(&self) -> PathBuf {
        self.get_passivate_path().join("coverage")
    }

    pub fn get_binary_path(&self) -> PathBuf {
        self.get_output_path().join("x86_64-pc-windows-msvc/debug")
    }

    fn runner_identifier(&self) -> PathBuf {
        match self.test_runner {
            TestRunnerImplementation::Cargo => PathBuf::from("cargo"),
            TestRunnerImplementation::Nextest => PathBuf::from("nextest")
        }
    }
}

pub fn test_run(test_runner: &mut ChangeEventHandler) -> Result<(), IoError> {
    let mock_event = ChangeEvent::File;
    test_runner.handle(mock_event, Cancellation::default());

    Ok(())
}
