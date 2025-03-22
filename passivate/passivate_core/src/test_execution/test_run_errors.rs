use std::io::Error as IoError;
use thiserror::Error;

use crate::actors::Cancelled;

#[derive(Error, Debug, Clone)]
pub enum TestRunError {
    #[error("{0}")]
    Io(String),

    #[error("test run cancelled")]
    Cancelled(#[from] Cancelled)
}

impl From<IoError> for TestRunError {
    fn from(value: IoError) -> Self {
        TestRunError::Io(value.to_string())
    }
}
