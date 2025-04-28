use std::io::Error as IoError;
use thiserror::Error;

use passivate_delegation::Cancelled;

#[derive(Error, Debug)]
pub enum TestRunError {
    #[error("{0}")]
    Io(#[from] IoError),

    #[error("test run cancelled")]
    Cancelled(#[from] Cancelled)
}
