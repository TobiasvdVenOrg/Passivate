use std::ffi::OsString;
use std::io::Error as IoError;
use camino::Utf8PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CargoWorkspaceError
{
    #[error("cargo metadata failed")]
    MetadataCommand(#[from] cargo_metadata::Error),

    #[error("io error during cargo metadata")]
    Io(#[from] IoError),

    #[error("could not find cargo.toml file")]
    TomlNotFound(Utf8PathBuf),

    #[error("could not find `Cargo.toml` in `{path:?}`, but found {found:?} please try to rename it to Cargo.toml")]
    IncorrectTomlCasing
    {
        path: Utf8PathBuf, found: OsString
    },

    #[error("format of `Cargo.toml` file was not UTF8")]
    NonUtf8
}
