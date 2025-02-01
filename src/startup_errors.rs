use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use passivate_core::passivate_notify::NotifyChangeEventsError;

#[derive(Debug)]
pub enum StartupError {
    MissingArgument(MissingArgumentError),
    NotifyChangeEvents(NotifyChangeEventsError)
}

#[derive(Debug)]
pub struct MissingArgumentError {
    pub argument: String
}

impl Display for StartupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StartupError::MissingArgument(missing_argument) => {
                write!(f, "missing argument: {}", missing_argument.argument)
            }
            StartupError::NotifyChangeEvents(notify_change_events) => {
                writeln!(f, "failed to start Notify for input project")?;
                writeln!(f, "")?;
                write!(f, "{}", notify_change_events)
            }
        }
    }
}

impl Error for StartupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StartupError::MissingArgument(_) => { None }
            StartupError::NotifyChangeEvents(notify_change_events) => { Some(notify_change_events) }
        }
    }
}

impl From<NotifyChangeEventsError> for StartupError {
    fn from(err: NotifyChangeEventsError) -> Self {
        StartupError::NotifyChangeEvents(err)
    }
}

impl From<MissingArgumentError> for StartupError {
    fn from(err: MissingArgumentError) -> Self {
        StartupError::MissingArgument(err)
    }
}