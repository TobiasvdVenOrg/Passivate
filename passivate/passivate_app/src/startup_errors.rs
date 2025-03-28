use std::error::Error;
use std::io::Error as IoError;
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::SendError;
use passivate_core::change_events::ChangeEvent;
use crate::passivate_notify::NotifyChangeEventsError;

#[derive(Debug)]
pub enum StartupError {
    MissingArgument(MissingArgumentError),
    NotifyChangeEvents(NotifyChangeEventsError),
    Channel(SendError<ChangeEvent>),
    DirectorySetup(IoError)
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
                writeln!(f)?;
                write!(f, "{}", notify_change_events)
            }
            StartupError::Channel(channel) => {
                writeln!(f, "failed to start Passivate test runner")?;
                writeln!(f)?;
                write!(f, "{}", channel)
            }
            StartupError::DirectorySetup(directory_setup_error) => {
                writeln!(f, "failed to initialize Passivate environment")?;
                writeln!(f)?;
                write!(f, "{}", directory_setup_error)
            },
        }
    }
}

impl Error for StartupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StartupError::MissingArgument(_) => { None }
            StartupError::NotifyChangeEvents(notify_change_events) => { Some(notify_change_events) }
            StartupError::Channel(channel) => { Some(channel) }
            StartupError::DirectorySetup(directory_setup_error) => Some(directory_setup_error),
        }
    }
}

impl From<NotifyChangeEventsError> for StartupError {
    fn from(error: NotifyChangeEventsError) -> Self {
        StartupError::NotifyChangeEvents(error)
    }
}

impl From<MissingArgumentError> for StartupError {
    fn from(error: MissingArgumentError) -> Self {
        StartupError::MissingArgument(error)
    }
}

impl From<SendError<ChangeEvent>> for StartupError {
    fn from(error: SendError<ChangeEvent>) -> Self {
        StartupError::Channel(error)
    }
}

impl From<IoError> for StartupError {
    fn from(error: IoError) -> Self {
        StartupError::DirectorySetup(error)
    }
}