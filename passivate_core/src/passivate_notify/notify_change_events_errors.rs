use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use notify::Error as NotifyError;

#[derive(Debug)]
pub enum NotifyChangeEventsError {
    InvalidPath(InvalidPathError)
}

#[derive(Debug)]
pub struct InvalidPathError {
    pub path: PathBuf,
    pub notify_error: NotifyError
}

impl NotifyChangeEventsError {
    pub fn invalid_path(path: &Path, notify_error: NotifyError) -> NotifyChangeEventsError {
        NotifyChangeEventsError::InvalidPath(InvalidPathError { path: path.to_path_buf(), notify_error })
    }

    fn try_absolute_path(relative_path: &Path) -> String {
        let canonicalize = fs::canonicalize(relative_path);

        match canonicalize {
            Ok(absolute_path) => {
                absolute_path.display().to_string()
            }
            Err(_) => {
                "[Not Found]".to_string()
            }
        }
    }

    fn try_working_dir() -> String {
        let current_dir = std::env::current_dir();

        match current_dir {
            Ok(ok) => {
                ok.display().to_string()
            }
            Err(_) => {
                "[Unknown]".to_string()
            }
        }
    }
}

impl Display for NotifyChangeEventsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            NotifyChangeEventsError::InvalidPath(invalid_path) => {
                writeln!(f, "input was: {}", invalid_path.path.display())?;
                writeln!(f, "full path was: {}", Self::try_absolute_path(&invalid_path.path))?;
                write!(f, "working directory: {}", Self::try_working_dir())
            }
        }
    }
}

impl Error for NotifyChangeEventsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            NotifyChangeEventsError::InvalidPath(invalid_path) => {
                Some(invalid_path.notify_error.source()?)
            }
        }
    }
}