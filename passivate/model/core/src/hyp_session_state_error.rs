use thiserror::*;

use crate::bridge::Bridge;
use crate::hyp_session_event::HypSessionEvent;

#[derive(Error, Debug, Clone)]
pub enum HypSessionStateError<TBridge: Bridge>
{
    #[error("session received an unexpect event")]
    UnexpectedEvent
    {
        event: HypSessionEvent<TBridge>
    }
}
