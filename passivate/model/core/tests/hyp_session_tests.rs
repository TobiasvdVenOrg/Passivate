#[macro_use]
extern crate assert_matches;

use std::ops::{Deref, DerefMut};

use itertools::assert_equal;
use passivate_model_core::bridge::{Bridge, HypSessionBridge, ProjectId};
use passivate_model_core::hyp_session::{HypSession, HypSessionActivity};
use passivate_model_core::hyp_session_event::{
    CompilationMessage,
    CompilationMessageKind,
    HypSessionEvent,
    ProjectCompilation
};
use passivate_model_core::hyp_session_state_error::HypSessionStateError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestSession(HypSession<TestSession>);

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestProjectInfo
{
    pub name: String
}

impl TestSession
{
    pub fn new() -> Self
    {
        Self(HypSession::<TestSession>::new())
    }
}

impl Deref for TestSession
{
    type Target = HypSession<TestSession>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl DerefMut for TestSession
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

impl TestProjectInfo
{
    pub fn new(name: impl Into<String>) -> Self
    {
        Self { name: name.into() }
    }
}

impl ProjectId for TestProjectInfo
{
    type TId = String;

    fn id(&self) -> &Self::TId
    {
        &self.name
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestHyp
{
    project: String,
    name: String
}

impl ProjectId for TestHyp
{
    type TId = String;

    fn id(&self) -> &Self::TId
    {
        &self.project
    }
}

impl TestHyp
{
    pub fn new(project: impl Into<String>, name: impl Into<String>) -> Self
    {
        Self {
            project: project.into(),
            name: name.into()
        }
    }
}

impl Bridge for TestSession
{
    type THypNode = TestHyp;
    type TProjectCompilation = ProjectCompilation<String>;
    type TProjectId = String;
    type TProjectInfo = TestProjectInfo;
    type TWorkspaceCompilation = CompilationMessage;
}

impl HypSessionBridge<TestSession> for TestSession
{
    fn start_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunStarted);
    }

    fn project_exists(&mut self, project: TestProjectInfo)
    {
        self.0.update(HypSessionEvent::ProjectExists(project));
    }

    fn workspace_compilation(&mut self, compilation: CompilationMessage)
    {
        self.0.update(HypSessionEvent::WorkspaceCompilation(compilation));
    }

    fn project_compilation(&mut self, compilation: ProjectCompilation<String>)
    {
        self.0.update(HypSessionEvent::ProjectCompilation(compilation));
    }

    fn hyp_node_exists(&mut self, hyp_node: TestHyp)
    {
        self.0.update(HypSessionEvent::HypNodeExists(hyp_node));
    }

    fn complete_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunCompleted);
    }
}

#[test]
pub fn default_session_has_no_hyps()
{
    let session = TestSession::new();

    assert_matches!(session.all_hyps().next(), None);
}

#[test]
pub fn default_session_is_idle()
{
    let session = TestSession::new();

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

    session.complete_run();

    assert_matches!(session.activity(), Ok(HypSessionActivity::Idle));
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

    let project_1 = TestProjectInfo::new("test_project_1");
    let project_2 = TestProjectInfo::new("test_project_2");

    session.project_exists(project_1.clone());
    session.project_exists(project_2.clone());

    assert_equal(session.project_infos(), [&project_1, &project_2]);
}

#[test]
pub fn last_workspace_compilation_is_most_recent()
{
    let mut session = new_started_session();

    session.workspace_compilation(CompilationMessage::new_info("example message"));
    session.workspace_compilation(CompilationMessage::new_warning("example warning"));

    assert_matches!(session.last_workspace_compilation(), Some(compilation_message) =>
    {
        assert_matches!(compilation_message.kind, CompilationMessageKind::Warning);
        assert_eq!(compilation_message.content, "example warning");
    });
}

#[test]
pub fn project_compilation_is_associated_with_project()
{
    let mut session = new_started_session();

    let project_1 = TestProjectInfo::new("test_project_1");
    let project_2 = TestProjectInfo::new("test_project_2");

    session.project_exists(project_1.clone());
    session.project_exists(project_2.clone());

    let example_project_compilation = ProjectCompilation {
        project_id: project_2.id().clone(),
        message: CompilationMessage::new_warning("example warning")
    };

    session.project_compilation(example_project_compilation.clone());

    let first_project = session.projects().next().unwrap();
    let second_project = session.projects().last().unwrap();

    assert_eq!(first_project.compilation.is_empty(), true);
    assert_equal(second_project.compilation.iter(), [&example_project_compilation]);
}

#[test]
pub fn compilation_for_unknown_project_is_error_state()
{
    let mut session = new_started_session();

    let example_project_compilation = ProjectCompilation {
        project_id: String::from("unknown project"),
        message: CompilationMessage::new_warning("example warning")
    };

    session.project_compilation(example_project_compilation.clone());

    let invalid_event = HypSessionEvent::ProjectCompilation(example_project_compilation);
    assert_matches!(session.activity(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedEvent { event } =>
        {
            assert_eq!(*event, invalid_event);
        });
    });
}

#[test]
pub fn hyp_becomes_part_of_project()
{
    let mut session = new_started_session();

    let project_info = TestProjectInfo::new("example_project");
    session.project_exists(project_info);

    let hyp = TestHyp::new("example_project", "example_hyp");
    session.hyp_node_exists(hyp.clone());

    let project = session.projects().next().unwrap();

    assert_equal(project.hyp_nodes.iter(), [&hyp]);
}

#[test]
pub fn hyp_for_unknown_project_is_error_state()
{
    let mut session = new_started_session();

    let hyp = TestHyp::new("invalid_project", "example_hyp");
    session.hyp_node_exists(hyp.clone());

    let invalid_event = HypSessionEvent::HypNodeExists(hyp);
    assert_matches!(session.activity(), Err(error) =>
    {
        assert_matches!(error, HypSessionStateError::UnexpectedEvent { event } =>
        {
            assert_eq!(*event, invalid_event);
        });
    });
}

fn new_started_session() -> TestSession
{
    let mut session = TestSession::new();

    session.start_run();

    session
}
