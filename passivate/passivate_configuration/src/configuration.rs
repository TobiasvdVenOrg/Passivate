use std::{io::Error as IoError, string::FromUtf8Error, sync::Arc};

use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Configuration
{
    pub coverage_enabled: bool,
    pub snapshots_path: Option<String>
}

#[derive(Error, Debug, Clone)]
pub enum ConfigurationLoadError
{
    #[error("failed to load configuration: {0}")]
    Io(#[from] Arc<IoError>),

    #[error("configuration data was not in UTF8 format: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("error deserializing toml: {0}")]
    Toml(#[from] toml::de::Error)
}

#[derive(Error, Debug, Clone)]
pub enum ConfigurationPersistError
{
    #[error("failed to persist configuration: {0}")]
    Io(#[from] Arc<IoError>),

    #[error("error serializing toml: {0}")]
    Toml(#[from] toml::ser::Error)
}

#[derive(Error, Debug)]
pub enum ConfigurationError
{
    #[error("failed to load configuration: {0}")]
    Load(#[from] ConfigurationLoadError),

    #[error("failed to persist configuration: {0}")]
    Persist(#[from] ConfigurationPersistError)
}
