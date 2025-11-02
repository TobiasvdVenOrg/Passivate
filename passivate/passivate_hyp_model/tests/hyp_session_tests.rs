#[macro_use] extern crate assert_matches;

use passivate_hyp_model::{hyp_run_state::HypRunState, hyp_session::HypSession};


#[test]
pub fn default_hyp_session_last_run_has_no_hyps()
{
    let session = HypSession::default();
    let last_run = session.last_run();

    assert!(last_run.hyps.is_empty());
}

#[test]
pub fn default_hyp_session_is_idle()
{
    let session = HypSession::default();

    let state = &session.current_run().state;

    assert_matches!(state, HypRunState::Idle);
}
