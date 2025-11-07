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
    #[error("hyp session received an unexpected transition from {from} to {to}")]
    UnexpectedStateChange
    {
        from: HypSessionState, to: HypSessionState
    }
}
