use std::io;

use passivate_delegation::Cancelled;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TestRunError
{
    #[error("{0}")]
    Io(String), // 'String' because std::io::Error is !Eq

    #[error("test run cancelled")]
    Cancelled(#[from] Cancelled),

    #[error("temp")]
    Temp
}

impl From<io::Error> for TestRunError
{
    fn from(value: io::Error) -> Self
    {
        TestRunError::Io(value.to_string())
    }
}
