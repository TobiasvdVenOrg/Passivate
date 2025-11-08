#[macro_use]
extern crate assert_matches;

use passivate_hyp_model::hyp_session::HypSession;
use passivate_hyp_model::hyp_session_event::HypSessionEvent;
use passivate_hyp_model::hyp_session_state::{HypSessionState, HypSessionStateError};

#[test]
pub fn default_session_has_no_hyps()
{
    let session = HypSession::new();

    assert_matches!(session.all_hyps().next(), None);
}

#[test]
pub fn default_session_is_idle()
{
    let session = HypSession::new();

    assert_matches!(session.state(), Ok(HypSessionState::Idle));
}

#[test]
pub fn started_session_is_running()
{
    let session = new_started_session();

    assert_matches!(session.state(), Ok(HypSessionState::Running));
}

#[test]
pub fn completed_session_is_idle()
{
    let mut session = new_started_session();

    session.update(HypSessionEvent::RunCompleted);

    assert_matches!(session.state(), Ok(HypSessionState::Idle));
}

#[test]
pub fn completing_an_idle_session_is_error_state()
{
    let mut session = HypSession::new();

    session.update(HypSessionEvent::RunCompleted);

    assert_matches!(session.state(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedCompletion);
    });
}

#[test]
pub fn starting_a_running_session_is_error_state()
{
    let mut session = new_started_session();

    session.update(HypSessionEvent::RunStarted);

    assert_matches!(session.state(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedStart);
    });
}

#[test]
pub fn new_errors_do_not_replace_original_error_state()
{
    let mut session = new_started_session();

    session.update(HypSessionEvent::RunStarted);
    session.update(HypSessionEvent::RunCompleted);

    assert_matches!(session.state(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedStart);
    });
}

#[test]
pub fn when_crate_starts_compiling_it_is_part_of_session()
{
    let mut session = new_started_session();

    // session.update(HypSessionEvent::CrateExists {})
}

fn new_started_session() -> HypSession
{
    let mut session = HypSession::new();

    session.update(HypSessionEvent::RunStarted);

    session
}
