use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use thiserror::*;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum HypSessionStateError<TBridge: Bridge>
{
    #[error("session received an unexpected event: {event}")]
    UnexpectedEvent
    {
        event: HypSessionEvent<TBridge>
    }
}
