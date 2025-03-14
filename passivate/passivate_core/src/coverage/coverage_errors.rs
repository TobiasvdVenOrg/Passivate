use std::io::ErrorKind as IoErrorKind;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CoverageError {
    #[error("grcov is not installed")]
    GrcovNotInstalled(IoErrorKind),

    #[error("coverage failed to generate for an unexpected reason")]
    FailedToGenerate(IoErrorKind),

    #[error("coverage may be inaccurate - cleaning previous output failed")]
    CleanIncomplete(IoErrorKind)
}
