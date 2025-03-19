use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use thiserror::Error;

use crate::actors::Cancelled;

#[derive(Error, Debug, Clone)]
pub enum TestRunError {
    #[error("test run failed")]
    Io(IoErrorKind),

    #[error("test run cancelled")]
    Cancelled(#[from] Cancelled)
}

impl From<IoError> for TestRunError {
    fn from(value: IoError) -> Self {
        TestRunError::Io(value.kind())
    }
}