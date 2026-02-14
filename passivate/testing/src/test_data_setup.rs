use std::fs;

use bon::bon;
use camino::Utf8PathBuf;
use passivate_configuration::default_paths::DefaultPaths;

use crate::path_resolution::{clean_directory, test_data_path, test_output_path};
use crate::test_snapshot_path::{TestSnapshotPath, TestSnapshotPathKind};

pub struct TestDataSetup
{
    output_path: Utf8PathBuf,
    workspace_path: Utf8PathBuf,
    base_output_path: Utf8PathBuf,
    base_workspace_path: Utf8PathBuf,
    override_snapshot_directories: Vec<TestSnapshotPath>
}

#[bon]
impl TestDataSetup
{
    #[builder]
    pub fn new(
        #[builder(start_fn, into)] output_path: Utf8PathBuf,
        #[builder(start_fn, into)] workspace_path: Utf8PathBuf,
        #[builder(default = test_output_path())] base_output_path: Utf8PathBuf,
        #[builder(default = test_data_path())] base_workspace_path: Utf8PathBuf,
        #[builder(default = Vec::new())] override_snapshot_directories: Vec<TestSnapshotPath>
    ) -> Self
    {
        Self {
            output_path,
            workspace_path,
            base_output_path,
            base_workspace_path,
            override_snapshot_directories
        }
    }

    pub fn clean_output(self) -> Self
    {
        let output_path = self.output_path();
        clean_directory(output_path);

        self
    }

    pub fn clean_snapshots(self) -> Self
    {
        let snapshot_directories = self.snapshot_directories();

        for snapshots_path in snapshot_directories
        {
            if fs::exists(&snapshots_path).expect("Failed to check if output_path exists!")
            {
                eprintln!("Cleaning snapshots_path: {:?}", snapshots_path);

                fs::remove_dir_all(&snapshots_path).expect("Failed to clear output path!")
            }
        }

        self
    }

    pub fn workspace_path(&self) -> Utf8PathBuf
    {
        self.base_workspace_path.join(&self.workspace_path)
    }

    pub fn output_path(&self) -> Utf8PathBuf
    {
        self.base_output_path.join(&self.output_path)
    }

    pub fn passivate_path(&self) -> Utf8PathBuf
    {
        self.output_path().join(".passivate")
    }

    pub fn coverage_path(&self) -> Utf8PathBuf
    {
        self.passivate_path().join("coverage")
    }

    pub fn binary_path(&self) -> Utf8PathBuf
    {
        self.output_path().join("debug")
    }

    pub fn paths(&self) -> DefaultPaths
    {
        DefaultPaths {
            root: self.workspace_path(),
            passivate: self.passivate_path()
        }
    }

    pub fn snapshot_directories(&self) -> Vec<Utf8PathBuf>
    {
        self.override_snapshot_directories
            .iter()
            .map(|p| {
                match &p
                {
                    TestSnapshotPath {
                        kind: TestSnapshotPathKind::Normal,
                        path
                    } => path.clone(),
                    TestSnapshotPath {
                        kind: TestSnapshotPathKind::RelativeToOutput,
                        path
                    } => self.output_path().join(path),
                    TestSnapshotPath {
                        kind: TestSnapshotPathKind::RelativeToWorkspace,
                        path
                    } => self.workspace_path().join(path)
                }
            })
            .collect()
    }
}
