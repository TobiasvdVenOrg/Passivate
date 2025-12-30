use std::io;

use camino::Utf8PathBuf;
use png::DecodingError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnapshotError
{
    #[error("snapshot format '{color_type:?}' is not supported: {path}")]
    Unsupported
    {
        color_type: png::ColorType, path: Utf8PathBuf
    },

    #[error("snapshot data did not match expected size: {path}")]
    InvalidData
    {
        path: Utf8PathBuf
    },

    #[error("io error occurred loading snapshot:\n{error}\n{path}")]
    Io
    {
        error: io::Error, path: Utf8PathBuf
    },

    #[error("decoding error {error} in snapshot: {path}")]
    Decoding
    {
        error: DecodingError, path: Utf8PathBuf
    }
}
