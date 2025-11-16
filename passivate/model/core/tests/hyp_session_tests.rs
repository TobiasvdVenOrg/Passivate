#[macro_use]
extern crate assert_matches;

use passivate_model_core::bridge::Bridge;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_event::HypSessionEvent;
use passivate_model_core::hyp_session_state::{HypSessionState, HypSessionStateError};

struct TestBridge;
struct TestProject;

impl Bridge for TestBridge
{
    type TProject = TestProject;
}

#[test]
pub fn default_session_has_no_hyps()
{
    let session = HypSession::<TestBridge>::new();

    assert_matches!(session.all_hyps().next(), None);
}

#[test]
pub fn default_session_is_idle()
{
    let session = HypSession::<TestBridge>::new();

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
    let mut session = HypSession::<TestBridge>::new();

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
pub fn project_existence_updates_session_while_idle()
{
    let mut session = HypSession::<TestBridge>::new();

    session.update(HypSessionEvent::ProjectExists(TestProject));
}

#[test]
pub fn crate_existence_updates_while_running_is_an_error()
{
    let mut session = new_started_session();

    // session.update(HypSessionEvent::CrateExists {})
}

fn new_started_session() -> HypSession<TestBridge>
{
    let mut session = HypSession::new();

    session.update(HypSessionEvent::RunStarted);

    session
}
