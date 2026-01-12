#[macro_use]
extern crate assert_matches;

use itertools::assert_equal;
use passivate_id_chain_tree::chain;
use passivate_model_bridge::hyp_session_bridge::*;
use passivate_model_bridge::hyp_session_event::{CompilationMessage, CompilationMessageKind, HypSessionEvent};
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_bridge::output_report::OutputReport;
use passivate_model_core::hyp_session_state_error::HypSessionStateError;
use passivate_testing::model::{TestHyp, TestHypKind, TestId, TestOutput, TestProject, TestSession};

#[test]
pub fn default_session_has_no_hyps()
{
    let session = TestSession::new();

    assert_matches!(session.hyps().iter().next(), None);
}

#[test]
pub fn default_session_is_in_unknown_state()
{
    let session = TestSession::new();

    assert_matches!(session.state(), HypState::Unknown);
}

#[test]
pub fn started_session_is_running()
{
    let session = new_started_session();

    assert_matches!(session.state(), HypState::Running);
}

#[test]
pub fn completed_empty_session_is_passed()
{
    let mut session = new_started_session();

    session.complete_run();

    assert_matches!(session.state(), HypState::Passed);
}

#[test]
pub fn completing_an_idle_session_is_error_state()
{
    let mut session = TestSession::new();

    session.complete_run();

    assert_matches!(session.activity(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedEvent { event } =>
        {
            assert_eq!(HypSessionEvent::RunCompleted, *event);
        });
    });
}

#[test]
pub fn starting_a_started_session_is_error_state()
{
    let mut session = new_started_session();

    session.start_run();

    assert_matches!(session.activity(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedEvent { event } =>
        {
            assert_eq!(HypSessionEvent::RunStarted, *event);
        });
    });
}

#[test]
pub fn new_errors_do_not_replace_original_error_state()
{
    let mut session = new_started_session();

    session.start_run();
    session.complete_run();

    assert_matches!(session.activity(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedEvent { event } =>
        {
            assert_eq!(HypSessionEvent::RunStarted, *event);
        });
    });
}

#[test]
pub fn project_existence_updates_session()
{
    let mut session = new_started_session();

    let project_1 = TestHypKind::Project(TestProject::new("test_project_1"));
    let project_2 = TestHypKind::Project(TestProject::new("test_project_2"));

    session.send_hyp(project_1.clone());
    session.send_hyp(project_2.clone());

    assert_equal(session.hyps().iter().map(|h| h.info()), [&project_1, &project_2]);
}

#[test]
pub fn last_session_output_is_most_recent()
{
    let mut session = new_started_session();

    session.send_output(OutputReport::new(
        TestId::empty(),
        TestOutput::Compilation(CompilationMessage::new_info("example message"))
    ));
    session.send_output(OutputReport::new(
        TestId::empty(),
        TestOutput::Compilation(CompilationMessage::new_warning("example warning"))
    ));

    assert_matches!(session.iter_output().last(), Some(output) =>
    {
        assert_matches!(output, TestOutput::Compilation(compilation) =>
        {
            assert_matches!(compilation.kind, CompilationMessageKind::Warning);
            assert_eq!(compilation.content, "example warning");
        });
    });
}

#[test]
pub fn output_for_unknown_hyp_is_error_state()
{
    let mut session = new_started_session();

    let nonexistent_id = TestId::from("a::b::c");
    let output = TestOutput::Compilation(CompilationMessage::new_info("example message"));
    let report = OutputReport::new(nonexistent_id.clone(), output.clone());

    session.send_output(report);

    let invalid_event = HypSessionEvent::Output(OutputReport::new(nonexistent_id, output));
    assert_matches!(session.activity(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedEvent { event } =>
        {
            assert_eq!(*event, invalid_event);
        });
    });
}

#[test]
pub fn hyp_becomes_part_of_parent()
{
    let mut session = new_started_session();

    let project_info = TestHypKind::Project(TestProject::new("example_project"));
    session.send_hyp(project_info);

    let hyp = TestHypKind::Hyp(TestHyp::new(TestId::from("example_project::example_hyp")));
    session.send_hyp(hyp.clone());

    let project_id = chain!("example_project");
    let project = session.hyps().entry(project_id).node_or_none().unwrap();

    assert_equal(project.iter_children().map(|c| c.info()), [&hyp]);
}

#[test]
pub fn run_error_leaves_session_completed_but_in_failed_state()
{
    let mut session = new_started_session();

    session.run_error(String::from(
        "An error occurred that affects the entire run (but is not a logic error)"
    ));

    assert_matches!(session.activity(), Ok(HypState::Failed));
}

fn new_started_session() -> TestSession
{
    let mut session = TestSession::new();

    session.start_run();

    session
}
