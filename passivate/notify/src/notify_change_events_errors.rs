use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use camino::{Utf8Path, Utf8PathBuf};
use notify::Error as NotifyError;

pub enum NotifyChangeEventsError
{
    InvalidPath
    {
        path: Utf8PathBuf, notify_error: NotifyError
    }
}

impl NotifyChangeEventsError
{
    pub fn invalid_path(path: &Utf8Path, notify_error: NotifyError) -> NotifyChangeEventsError
    {
        NotifyChangeEventsError::InvalidPath {
            path: path.to_path_buf(),
            notify_error
        }
    }

    fn try_absolute_path(relative_path: &Utf8Path) -> String
    {
        let canonicalize = dunce::canonicalize(relative_path);

        match canonicalize
        {
            Ok(absolute_path) => absolute_path.display().to_string(),
            Err(_) => "[Not Found]".to_string()
        }
    }

    fn try_working_dir() -> String
    {
        let current_dir = std::env::current_dir();

        match current_dir
        {
            Ok(ok) => ok.display().to_string(),
            Err(_) => "[Unknown]".to_string()
        }
    }
}

impl Display for NotifyChangeEventsError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match &self
        {
            NotifyChangeEventsError::InvalidPath { path, notify_error: _ } =>
            {
                writeln!(f, "input was: {}", path)?;
                writeln!(f, "full path was: {}", Self::try_absolute_path(path))?;
                write!(f, "working directory: {}", Self::try_working_dir())
            }
        }
    }
}

impl Debug for NotifyChangeEventsError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self)
    }
}

impl Error for NotifyChangeEventsError
{
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        match &self
        {
            NotifyChangeEventsError::InvalidPath { path: _, notify_error } => Some(notify_error.source()?)
        }
    }
}
