use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use bon::bon;
use passivate_delegation::Tx;

use crate::configuration::{ConfigurationManager, PassivateConfig, TestRunnerImplementation};
use crate::coverage::CoverageStatus;
use crate::passivate_grcov::Grcov;
use crate::test_execution::{ParseOutput, TestRunHandler, TestRunProcessor, TestRunner, build_test_output_parser};
use crate::test_run_model::TestRun;

pub struct TestRunSetup
{
    test_runner: TestRunnerImplementation,
    output_path: PathBuf,
    workspace_path: PathBuf,
    base_output_path: PathBuf,
    base_workspace_path: PathBuf,
    tests_status_sender: Tx<TestRun>,
    coverage_sender: Tx<CoverageStatus>,
    coverage_enabled: bool
}

pub fn test_output_path() -> PathBuf
{
    dunce::canonicalize(PathBuf::from("../../test_output")).expect("test output path did not exist!")
}

pub fn test_data_path() -> PathBuf
{
    dunce::canonicalize(PathBuf::from("../../test_data")).expect("test data path did not exist!")
}

pub fn get_default_workspace_path<P>(workspace_path: P) -> PathBuf
where
    P: AsRef<Path>
{
    test_data_path().join(workspace_path)
}

#[bon]
impl TestRunSetup
{
    #[builder]
    pub fn new(
        #[builder(start_fn)] test_runner_implementation: TestRunnerImplementation,
        #[builder(start_fn, into)] output_path: PathBuf,
        #[builder(start_fn, into)] workspace_path: PathBuf,
        #[builder(default = test_output_path())] base_output_path: PathBuf,
        #[builder(default = test_data_path())] base_workspace_path: PathBuf,
        #[builder(default = false)] coverage_enabled: bool,
        #[builder(default = Tx::stub())] tests_status_sender: Tx<TestRun>,
        #[builder(default = Tx::stub())] coverage_sender: Tx<CoverageStatus>
    ) -> Self
    {
        Self {
            test_runner: test_runner_implementation,
            output_path,
            workspace_path,
            base_output_path,
            base_workspace_path,
            coverage_enabled,
            tests_status_sender,
            coverage_sender
        }
    }

    #[builder]
    pub fn cargo(
        #[builder(start_fn, into)] output_path: PathBuf,
        #[builder(start_fn, into)] workspace_path: PathBuf,
        #[builder(default = test_output_path())] base_output_path: PathBuf,
        #[builder(default = test_data_path())] base_workspace_path: PathBuf,
        #[builder(default = false)] coverage_enabled: bool,
        #[builder(default = Tx::stub())] tests_status_sender: Tx<TestRun>,
        #[builder(default = Tx::stub())] coverage_sender: Tx<CoverageStatus>
    ) -> Self
    {
        Self {
            test_runner: TestRunnerImplementation::Cargo,
            output_path,
            workspace_path,
            base_output_path,
            base_workspace_path,
            coverage_enabled,
            tests_status_sender,
            coverage_sender
        }
    }

    pub fn build_grcov(&self) -> Grcov
    {
        Grcov::builder()
            .workspace_path(self.get_workspace_path())
            .output_path(self.get_coverage_path())
            .binary_path(self.get_binary_path())
            .build()
    }

    pub fn build_test_run_processor(&self) -> TestRunProcessor
    {
        #[cfg(target_os = "windows")]
        let target = OsString::from("x86_64-pc-windows-msvc");

        #[cfg(target_os = "linux")]
        let target = OsString::from("aarch64-unknown-linux-gnu");

        let test_runner = TestRunner::builder()
            .target(target)
            .working_dir(self.get_workspace_path().clone())
            .target_dir(self.get_output_path().clone())
            .coverage_output_dir(self.get_coverage_path().clone())
            .build();

        let parser: Box<dyn ParseOutput + Send> = build_test_output_parser(&self.test_runner);
        TestRunProcessor::new(Box::new(test_runner), parser)
    }

    pub fn build_test_run_handler(self) -> TestRunHandler
    {
        let processor = self.build_test_run_processor();

        let grcov = self.build_grcov();

        let configuration = ConfigurationManager::new(
            PassivateConfig {
                coverage_enabled: self.coverage_enabled,
                snapshots_path: Some(self.get_snapshots_path().to_str().unwrap().to_string())
            },
            Tx::stub()
        );

        TestRunHandler::builder()
            .runner(processor)
            .coverage(Box::new(grcov))
            .tests_status_sender(self.tests_status_sender)
            .coverage_status_sender(self.coverage_sender)
            .log(Tx::stub())
            .configuration(configuration)
            .build()
    }

    pub fn clean_output(self) -> Self
    {
        let output_path = self.get_output_path();

        if fs::exists(&output_path).expect("Failed to check if output_path exists!")
        {
            eprintln!("Cleaning: {:?}", output_path);

            fs::remove_dir_all(&output_path).expect("Failed to clear output path!")
        }

        self
    }

    pub fn clean_snapshots(self) -> Self
    {
        let snapshots_path = self.get_snapshots_path();

        if fs::exists(&snapshots_path).expect("Failed to check if output_path exists!")
        {
            eprintln!("Cleaning snapshots_path: {:?}", snapshots_path);

            fs::remove_dir_all(&snapshots_path).expect("Failed to clear output path!")
        }

        self
    }

    pub fn get_workspace_path(&self) -> PathBuf
    {
        self.base_workspace_path.join(&self.workspace_path)
    }

    pub fn get_output_path(&self) -> PathBuf
    {
        self.base_output_path.join(&self.output_path).join(self.runner_identifier())
    }

    pub fn get_passivate_path(&self) -> PathBuf
    {
        self.get_output_path().join(".passivate")
    }

    pub fn get_coverage_path(&self) -> PathBuf
    {
        self.get_passivate_path().join("coverage")
    }

    pub fn get_binary_path(&self) -> PathBuf
    {
        self.get_output_path().join("x86_64-pc-windows-msvc/debug")
    }

    pub fn get_snapshots_path(&self) -> PathBuf
    {
        self.get_workspace_path().join("tests").join("snapshots")
    }

    fn runner_identifier(&self) -> PathBuf
    {
        match self.test_runner
        {
            TestRunnerImplementation::Cargo => PathBuf::from("cargo"),
            TestRunnerImplementation::Nextest => PathBuf::from("nextest")
        }
    }
}
