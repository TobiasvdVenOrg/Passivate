use std::fmt::Display;

use thiserror::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum HypSessionState
{
    #[default]
    Idle,
    Running
}

impl Display for HypSessionState
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{self:?}")
    }
}

#[derive(Error, Debug, Clone)]
pub enum HypSessionStateError
{
    #[error("session was started unexpectedly, it was already running")]
    UnexpectedStart,

    #[error("session was completed unexpectedly, it was already idle")]
    UnexpectedCompletion
}
