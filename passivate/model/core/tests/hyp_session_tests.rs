#[macro_use]
extern crate assert_matches;

use itertools::assert_equal;
use passivate_model_core::bridge::{Bridge, ProjectId};
use passivate_model_core::hyp_session::{HypSession, HypSessionActivity};
use passivate_model_core::hyp_session_change::HypSessionChange;
use passivate_model_core::hyp_session_event::{
    CompilationMessage,
    CompilationMessageKind,
    HypSessionEvent,
    ProjectCompilation
};
use passivate_model_core::hyp_session_state_error::HypSessionStateError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestBridge;

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestProject
{
    pub name: String
}

impl TestProject
{
    pub fn new(name: impl Into<String>) -> Self
    {
        Self { name: name.into() }
    }
}

impl ProjectId for TestProject
{
    type T = String;

    fn id(&self) -> &Self::T
    {
        &self.name
    }
}

impl Bridge for TestBridge
{
    type TProject = TestProject;
    type TProjectCompilation = ProjectCompilation<String>;
    type TProjectId = String;
    type TWorkspaceCompilation = CompilationMessage;
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

    assert_matches!(session.activity(), Ok(HypSessionActivity::Idle));
}

#[test]
pub fn started_session_is_running()
{
    let session = new_started_session();

    assert_matches!(session.activity(), Ok(HypSessionActivity::Running));
}

#[test]
pub fn completed_session_is_idle()
{
    let mut session = new_started_session();

    session.update(HypSessionEvent::RunCompleted);

    assert_matches!(session.activity(), Ok(HypSessionActivity::Idle));
}

#[test]
pub fn completing_an_idle_session_is_error_state()
{
    let mut session = HypSession::<TestBridge>::new();

    session.update(HypSessionEvent::RunCompleted);

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

    session.update(HypSessionEvent::RunStarted);

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

    session.update(HypSessionEvent::RunStarted);
    session.update(HypSessionEvent::RunCompleted);

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

    let project_1 = TestProject::new("test_project_1");
    let project_2 = TestProject::new("test_project_2");

    let change_1 = session.update(HypSessionEvent::ProjectExists(project_1.clone()));
    assert_matches!(change_1, Some(HypSessionChange::NewProject(new_1)) =>
    {
        assert_eq!(*new_1, project_1);
    });

    let change_2 = session.update(HypSessionEvent::ProjectExists(project_2.clone()));
    assert_matches!(change_2, Some(HypSessionChange::NewProject(new_2)) =>
    {
        assert_eq!(*new_2, project_2);
    });

    assert_equal(session.projects(), [&project_1, &project_2]);
}

#[test]
pub fn last_workspace_compilation_is_most_recent()
{
    let mut session = new_started_session();

    session.update(HypSessionEvent::WorkspaceCompilation(CompilationMessage::new_info(
        "example message"
    )));

    session.update(HypSessionEvent::WorkspaceCompilation(CompilationMessage::new_warning(
        "example warning"
    )));

    assert_matches!(session.last_workspace_compilation(), Some(compilation_message) =>
    {
        assert_matches!(compilation_message.kind, CompilationMessageKind::Warning);
        assert_eq!(compilation_message.content, "example warning");
    });
}

fn new_started_session() -> HypSession<TestBridge>
{
    let mut session = HypSession::new();

    session.update(HypSessionEvent::RunStarted);

    session
}
