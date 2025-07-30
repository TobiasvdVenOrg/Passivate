use std::io::ErrorKind as IoErrorKind;
use std::path::PathBuf;

use passivate_delegation::Cancelled;
use thiserror::Error;

use crate::passivate_cargo::CargoWorkspaceError;

#[derive(Error, Debug)]
pub enum CoverageError
{
    #[error("grcov is not installed")]
    GrcovNotInstalled(IoErrorKind),

    #[error("coverage failed to generate for an unexpected reason")]
    FailedToGenerate(IoErrorKind),

    #[error("coverage may be inaccurate - cleaning previous output failed")]
    CleanIncomplete(IoErrorKind),

    #[error("coverage did not run - no profraw files were present")]
    NoProfrawFiles(NoProfrawFilesError),

    #[error("failed to read covdir output")]
    CovdirRead(IoErrorKind),

    #[error("failed to parse covdir output")]
    CovdirParse(String),

    #[error("coverage was cancelled")]
    Cancelled(#[from] Cancelled),

    #[error("unexpected failure parsing workspace metadata")]
    Workspace(#[from] CargoWorkspaceError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoProfrawFilesKind
{
    Io(IoErrorKind),
    NoProfrawFilesExist
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoProfrawFilesError
{
    pub expected_path: PathBuf,
    pub kind: NoProfrawFilesKind
}
