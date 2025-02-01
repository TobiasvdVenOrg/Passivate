use std::result::Result as StdResult;
use notify::Error as NotifyError;

pub struct NotifyErrorInfo {
    pub notify_error: NotifyError,
    pub input_path: String
}

pub enum ErrorType {
    Notify(NotifyErrorInfo)
}

pub struct PassivateError {
    pub error_type: ErrorType
}

impl PassivateError {
    pub fn notify(error: NotifyError, input_path: &str) -> Self {
        PassivateError { error_type: ErrorType::Notify(NotifyErrorInfo { notify_error: error, input_path: input_path.to_string() })}
    }
}