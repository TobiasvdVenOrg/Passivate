use std::io;
use std::sync::Arc;

use passivate_delegation::Cancelled;
use thiserror::Error;

use crate::nextest_error::NextestError;

#[derive(Error, Debug)]
pub enum HypRunError
{
    #[error("{0}")]
    Io(String), // 'String' because std::io::Error is !Eq

    #[error("test run cancelled")]
    Cancelled(#[from] Cancelled),

    #[error("nextest error: {0}")]
    Nextest(#[from] Arc<NextestError>),

    #[error("guppy error: {0}")]
    Guppy(#[from] guppy::Error)
}

impl From<io::Error> for HypRunError
{
    fn from(value: io::Error) -> Self
    {
        HypRunError::Io(value.to_string())
    }
}

impl From<NextestError> for HypRunError
{
    fn from(value: NextestError) -> Self
    {
        HypRunError::Nextest(Arc::new(value))
    }
}
