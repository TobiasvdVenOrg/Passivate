use std::ffi::OsString;
use std::fs;

use bon::bon;
use camino::{Utf8Path, Utf8PathBuf};
use passivate_delegation::Tx;

use crate::configuration::{ConfigurationManager, PassivateConfig};
use crate::coverage::CoverageStatus;
use crate::passivate_grcov::Grcov;
use crate::passivate_nextest::NextestParser;
use crate::test_execution::{TestRunHandler, TestRunner};
use crate::test_run_model::TestRun;

pub struct TestRunSetup
{
    output_path: Utf8PathBuf,
    workspace_path: Utf8PathBuf,
    base_output_path: Utf8PathBuf,
    base_workspace_path: Utf8PathBuf,
    tests_status_sender: Tx<TestRun>,
    coverage_sender: Tx<CoverageStatus>,
    coverage_enabled: bool
}

pub fn test_output_path() -> Utf8PathBuf
{
    Utf8PathBuf::from_path_buf(dunce::canonicalize(Utf8PathBuf::from("../../test_output")).expect("test output path did not exist!")).expect("expected utf8 path")
}

pub fn test_data_path() -> Utf8PathBuf
{
    Utf8PathBuf::from_path_buf(dunce::canonicalize(Utf8PathBuf::from("../../test_data")).expect("test data path did not exist!")).expect("expected utf8 path")
}

pub fn get_default_workspace_path<P>(workspace_path: P) -> Utf8PathBuf
where
    P: AsRef<Utf8Path>
{
    test_data_path().join(workspace_path)
}

#[bon]
impl TestRunSetup
{
    #[builder]
    pub fn new(
        #[builder(start_fn, into)] output_path: Utf8PathBuf,
        #[builder(start_fn, into)] workspace_path: Utf8PathBuf,
        #[builder(default = test_output_path())] base_output_path: Utf8PathBuf,
        #[builder(default = test_data_path())] base_workspace_path: Utf8PathBuf,
        #[builder(default = false)] coverage_enabled: bool,
        #[builder(default = Tx::stub())] tests_status_sender: Tx<TestRun>,
        #[builder(default = Tx::stub())] coverage_sender: Tx<CoverageStatus>
    ) -> Self
    {
        Self {
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

    pub fn build_test_runner(&self) -> TestRunner
    {
        #[cfg(target_os = "windows")]
        let target = OsString::from("x86_64-pc-windows-msvc");

        #[cfg(target_os = "linux")]
        let target = OsString::from("aarch64-unknown-linux-gnu");

        TestRunner::new(target, self.get_workspace_path().clone(), self.get_output_path().clone(), self.get_coverage_path().clone(), TestRun::default())
    }

    pub fn build_test_run_handler(self) -> TestRunHandler
    {
        let runner = self.build_test_runner();

        let grcov = self.build_grcov();

        let configuration = ConfigurationManager::new(
            PassivateConfig {
                coverage_enabled: self.coverage_enabled,
                snapshots_path: Some(self.get_snapshots_path().to_string())
            },
            Tx::stub()
        );

        TestRunHandler::builder()
            .runner(runner)
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

    pub fn get_workspace_path(&self) -> Utf8PathBuf
    {
        self.base_workspace_path.join(&self.workspace_path)
    }

    pub fn get_output_path(&self) -> Utf8PathBuf
    {
        self.base_output_path.join(&self.output_path)
    }

    pub fn get_passivate_path(&self) -> Utf8PathBuf
    {
        self.get_output_path().join(".passivate")
    }

    pub fn get_coverage_path(&self) -> Utf8PathBuf
    {
        self.get_passivate_path().join("coverage")
    }

    pub fn get_binary_path(&self) -> Utf8PathBuf
    {
        self.get_output_path().join("x86_64-pc-windows-msvc/debug")
    }

    pub fn get_snapshots_path(&self) -> Utf8PathBuf
    {
        self.get_workspace_path().join("tests").join("snapshots")
    }
}
