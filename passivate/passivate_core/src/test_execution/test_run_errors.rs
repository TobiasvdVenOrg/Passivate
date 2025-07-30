use std::io::Error as IoError;

use passivate_delegation::Cancelled;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestRunError
{
    #[error("{0}")]
    Io(#[from] IoError),

    #[error("test run cancelled")]
    Cancelled(#[from] Cancelled)
}
