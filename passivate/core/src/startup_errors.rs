use std::fmt::Debug;
use std::io;
use std::sync::mpsc::SendError;

use passivate_configuration::configuration_errors::ConfigurationError;
use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_notify::notify_change_events_errors::NotifyChangeEventsError;
use passivate_run_rust::model::RustBridge;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StartupError
{
    #[error("missing argument: {argument}")]
    MissingArgument
    {
        argument: String
    },
    #[error("notify failed")]
    NotifyChangeEvents(#[from] NotifyChangeEventsError),
    #[error("channel error")]
    Channel(SendError<HypRunRequest<RustBridge>>),
    #[error("logger failed")]
    Logger(log::SetLoggerError),
    #[error("logger already initialized")]
    LoggerAlreadyInitialized,
    #[error("configuration error")]
    PassivateConfiguration(#[from] ConfigurationError),
    #[error("invalid UTF8")]
    Utf8(String),
    #[error("IO error")]
    Io(#[from] io::Error)
}
