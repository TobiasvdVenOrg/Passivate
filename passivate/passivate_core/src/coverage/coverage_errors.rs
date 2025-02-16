use std::io::Error as IoError;


pub enum CoverageError {
    GrcovNotInstalled(GrcovNotInstalledCoverageError),
    Io(IoError)
}

pub struct GrcovNotInstalledCoverageError {

}

impl From<IoError> for CoverageError {
    fn from(err: IoError) -> Self {
        CoverageError::Io(err)
    }
}