#[macro_use]
extern crate assert_matches;

use passivate_hyp_model::hyp_session::HypSession;
use passivate_hyp_model::hyp_session_event::HypSessionEvent;
use passivate_hyp_model::hyp_session_state::HypSessionState;

#[test]
pub fn default_session_has_no_hyps()
{
    let session = HypSession::default();

    assert!(session.no_hyps());
}

#[test]
pub fn default_session_is_idle()
{
    let session = HypSession::default();

    assert_matches!(session.state(), HypSessionState::Idle);
}

#[test]
pub fn started_session_is_running()
{
    let mut session = HypSession::default();

    session.update(HypSessionEvent::RunStarted);

    assert_matches!(session.state(), HypSessionState::Running);
}

#[test]
pub fn completed_session_is_idle()
{
    let mut session = HypSession::default();

    session.update(HypSessionEvent::RunStarted);
    session.update(HypSessionEvent::RunCompleted);

    assert_matches!(session.state(), HypSessionState::Idle);
}
