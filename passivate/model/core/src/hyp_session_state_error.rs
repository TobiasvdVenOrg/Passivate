use passivate_model_bridge::Bridge;
use thiserror::*;

use crate::hyp_session_event::HypSessionEvent;

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum HypSessionStateError<TBridge: Bridge>
{
    #[error("session received an unexpected event")]
    UnexpectedEvent
    {
        event: HypSessionEvent<TBridge>
    }
}
