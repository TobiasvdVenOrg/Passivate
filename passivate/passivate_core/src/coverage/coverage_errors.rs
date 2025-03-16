use std::{io::ErrorKind as IoErrorKind, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CoverageError {
    #[error("grcov is not installed")]
    GrcovNotInstalled(IoErrorKind),

    #[error("coverage failed to generate for an unexpected reason")]
    FailedToGenerate(IoErrorKind),

    #[error("coverage may be inaccurate - cleaning previous output failed")]
    CleanIncomplete(IoErrorKind),

    #[error("coverage did not run - no profraw files were present")]
    NoProfrawFiles(NoProfrawFilesError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoProfrawFilesKind {
    Io(IoErrorKind),
    NoProfrawFilesExist
}

#[derive(Debug, Clone)]
pub struct NoProfrawFilesError {
    pub expected_path: PathBuf,
    pub kind: NoProfrawFilesKind
}