use camino::Utf8PathBuf;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, Parser)]
pub struct PassivateConfiguration
{
    pub target_dir: Option<Utf8PathBuf>,
    pub coverage_enabled: bool,
    pub snapshot_directories: Vec<Utf8PathBuf>
}

impl PassivateConfiguration
{
    pub fn add_snapshot_directory(&mut self, snapshot_directory: Utf8PathBuf)
    {
        self.snapshot_directories.push(snapshot_directory);
    }
}

// TODO: Generate this
#[derive(Debug)]
pub enum ConfigurationChange
{
    TargetDir(Option<Utf8PathBuf>),
    CoverageEnabled(bool),
    SnapshotDirectories(Vec<Utf8PathBuf>),
    AddSnapshotDirectory(Utf8PathBuf)
}

impl PassivateConfiguration
{
    pub fn change(&mut self, change: ConfigurationChange)
    {
        match change
        {
            ConfigurationChange::TargetDir(target_dir) => self.target_dir = target_dir,
            ConfigurationChange::CoverageEnabled(coverage_enabled) => self.coverage_enabled = coverage_enabled,
            ConfigurationChange::SnapshotDirectories(snapshot_directories) => self.snapshot_directories = snapshot_directories,
            ConfigurationChange::AddSnapshotDirectory(snapshot_directory) => self.add_snapshot_directory(snapshot_directory)
        }
    }
}
