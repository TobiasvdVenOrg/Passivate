use std::fmt::Display;

use thiserror::*;

use crate::bridge::Bridge;
use crate::hyp_session_event::HypSessionEvent;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum HypSessionState
{
    #[default]
    Idle,
    Starting,
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
pub enum HypSessionStateError<TBridge: Bridge>
{
    #[error("session was received an unexpect event,")]
    UnexpectedEvent
    {
        state: HypSessionState,
        event: HypSessionEvent<TBridge>
    }
}
