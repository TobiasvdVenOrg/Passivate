use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum RunTestsError {
    Io(IoError),
    Utf8(FromUtf8Error)
}

impl From<IoError> for RunTestsError {
    fn from(err: IoError) -> Self {
        RunTestsError::Io(err)
    }
}

impl From<FromUtf8Error> for RunTestsError {
    fn from(err: FromUtf8Error) -> Self {
        RunTestsError::Utf8(err)
    }
}
