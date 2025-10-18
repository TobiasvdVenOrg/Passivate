use std::{string::FromUtf8Error, sync::Arc};

use camino::Utf8PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ConfigurationLoadError
{
    #[error("failed to load configuration: {0}")]
    Io(#[from] Arc<std::io::Error>),

    #[error("configuration data was not in UTF8 format: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("error deserializing toml: {0}")]
    Toml(#[from] toml::de::Error)
}

#[derive(Error, Debug, Clone)]
pub enum ConfigurationPersistError
{
    #[error("failed to persist configuration: {0}")]
    Io(#[from] Arc<std::io::Error>),

    #[error("error serializing toml: {0}")]
    Toml(#[from] toml::ser::Error),

    #[error("invalid configuration path: {0}")]
    Path(Utf8PathBuf)
}

#[derive(Error, Debug)]
pub enum ConfigurationError
{
    #[error("failed to load configuration: {0}")]
    Load(#[from] ConfigurationLoadError),

    #[error("failed to persist configuration: {0}")]
    Persist(#[from] ConfigurationPersistError)
}