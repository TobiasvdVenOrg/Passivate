use std::result::Result as StdResult;
use notify::Error as NotifyError;

pub struct MissingArgumentError {
    pub argument: String
}

pub struct NotifyErrorInfo {
    pub notify_error: NotifyError,
    pub input_path: String
}

pub enum ErrorType {
    Notify(NotifyErrorInfo),
    MissingArgument(MissingArgumentError),
}

pub struct PassivateError {
    pub error_type: ErrorType
}

pub type Result<T> = StdResult<T, PassivateError>;

impl PassivateError {
    pub fn missing_argument(argument: &str) -> Self {
        PassivateError { error_type: ErrorType::MissingArgument(MissingArgumentError { argument: argument.to_string() })}
    }

    pub fn notify(error: NotifyError, input_path: &str) -> Self {
        PassivateError { error_type: ErrorType::Notify(NotifyErrorInfo { notify_error: error, input_path: input_path.to_string() })}
    }
}