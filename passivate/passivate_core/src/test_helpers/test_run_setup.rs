use std::ffi::OsString;
use std::fs;

use bon::bon;
use camino::Utf8PathBuf;
use passivate_configuration::configuration::PassivateConfiguration;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_delegation::Tx;
use passivate_hyp_model::hyp_run_events::HypRunEvent;
use passivate_testing::path_resolution::{clean_directory, test_data_path, test_output_path};

use crate::coverage::CoverageStatus;
use crate::passivate_grcov::Grcov;
use crate::test_execution::{TestRunHandler, TestRunner};
use crate::test_helpers::test_snapshot_path::{TestSnapshotPath, TestSnapshotPathKind};

pub struct TestRunSetup
{
    output_path: Utf8PathBuf,
    workspace_path: Utf8PathBuf,
    base_output_path: Utf8PathBuf,
    base_workspace_path: Utf8PathBuf,
    hyp_run_tx: Tx<HypRunEvent>,
    coverage_sender: Tx<CoverageStatus>,
    coverage_enabled: bool,
    override_snapshot_path: TestSnapshotPath
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
        #[builder(default = Tx::stub())] hyp_run_tx: Tx<HypRunEvent>,
        #[builder(default = Tx::stub())] coverage_sender: Tx<CoverageStatus>,
        #[builder(default = TestSnapshotPath::default())] override_snapshot_path: TestSnapshotPath
    ) -> Self
    {
        Self {
            output_path,
            workspace_path,
            base_output_path,
            base_workspace_path,
            coverage_enabled,
            hyp_run_tx,
            coverage_sender,
            override_snapshot_path
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

        TestRunner::new(
            target,
            self.get_workspace_path().clone(),
            self.get_output_path().clone(),
            self.get_coverage_path().clone()
        )
    }

    pub fn build_test_run_handler(self) -> TestRunHandler
    {
        let runner = self.build_test_runner();

        let grcov = self.build_grcov();

        let configuration = ConfigurationManager::new(
            PassivateConfiguration {
                coverage_enabled: self.coverage_enabled,
                snapshots_path: Some(self.get_snapshots_path().to_string())
            },
            Tx::stub()
        );

        TestRunHandler::builder()
            .runner(runner)
            .coverage(Box::new(grcov))
            .hyp_run_tx(self.hyp_run_tx)
            .coverage_status_sender(self.coverage_sender)
            .configuration(configuration)
            .build()
    }

    pub fn clean_output(self) -> Self
    {
        let output_path = self.get_output_path();
        clean_directory(output_path);

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
        self.get_output_path().join("debug")
    }

    pub fn get_snapshots_path(&self) -> Utf8PathBuf
    {
        match &self.override_snapshot_path
        {
            TestSnapshotPath { kind: TestSnapshotPathKind::Normal, path } => path.clone(),
            TestSnapshotPath { kind: TestSnapshotPathKind::RelativeToOutput, path } => self.get_output_path().join(path),
            TestSnapshotPath { kind: TestSnapshotPathKind::RelativeToWorkspace, path }=> self.get_workspace_path().join(path),
        }
    }
}
