use std::io::Error as IoError;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunTestsError {
    #[error("failed to start test runner process")]
    Process(#[from] IoError),

    #[error("test runner output was not valid UTF8")]
    Output(#[from] FromUtf8Error)
}
