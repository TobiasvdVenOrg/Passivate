#[macro_use]
extern crate assert_matches;

use std::ops::{Deref, DerefMut};

use itertools::assert_equal;
use passivate_id_chain_tree::chain;
use passivate_id_chain_tree::depth::Depth;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::{Bridge, HypSessionBridge, OutputReport};
use passivate_model_core::hyp_session::{HypSession, HypSessionActivity};
use passivate_model_core::hyp_session_event::{CompilationMessage, CompilationMessageKind, HypSessionEvent};
use passivate_model_core::hyp_session_state_error::HypSessionStateError;

#[derive(Debug, PartialEq, Eq)]
struct TestSession(HypSession<TestSession>);

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

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestId(Vec<String>);

impl TestId
{
    pub fn empty() -> Self
    {
        Self(Vec::new())
    }
}

impl IdChain for TestId
{
    type Link = String;

    fn chain(&self) -> &[Self::Link]
    {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TestHypKind
{
    Hyp(TestHyp),
    Project(TestProject)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestHyp
{
    id: TestId
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestProject
{
    id: TestId
}

impl TestProject
{
    fn new(id: impl Into<TestId>) -> Self
    {
        Self { id: id.into() }
    }
}

impl<T: Into<String>> From<T> for TestId
{
    fn from(value: T) -> Self
    {
        Self(vec![value.into()])
    }
}

impl IdChain for TestHypKind
{
    type Link = String;

    fn chain(&self) -> &[Self::Link]
    {
        match self
        {
            TestHypKind::Hyp(test_hyp) => test_hyp.id.chain(),
            TestHypKind::Project(test_project) => test_project.id.chain()
        }
    }
}

impl TestHyp
{
    pub fn new(name: impl Into<String>) -> Self
    {
        Self {
            id: TestId(name.into().split("::").map(String::from).collect())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TestOutput
{
    Compilation(CompilationMessage)
}

impl IdChain for TestOutput
{
    type Link = String;

    fn chain(&self) -> &[Self::Link]
    {
        self.id.chain()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestBridge;

impl Bridge for TestSession
{
    type HypInfo = TestHypKind;
    type Id = TestId;
    type IdLink = String;
    type Output = TestOutput;
}

impl HypSessionBridge<TestSession> for TestSession
{
    fn start_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunStarted);
    }

    fn output(&mut self, report: OutputReport<TestOutput>)
    {
        self.0.update(HypSessionEvent::Output(report));
    }

    fn hyp(&mut self, hyp: TestHypKind)
    {
        self.0.update(HypSessionEvent::HypExists(hyp));
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

    assert_matches!(session.hyps().next(), None);
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

    let project_1 = TestProject::new("test_project_1");
    let project_2 = TestProject::new("test_project_2");

    assert_equal(session.hyps(), [(Depth::new(0), &project_1), (Depth::new(0), &project_2)]);
}

#[test]
pub fn last_session_output_is_most_recent()
{
    let mut session = new_started_session();

    session.output(OutputReport::new(
        TestId::empty(),
        TestOutput::Compilation(CompilationMessage::new_info("example message"))
    ));
    session.output(OutputReport::new(
        TestId::empty(),
        TestOutput::Compilation(CompilationMessage::new_warning("example warning"))
    ));

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
pub fn output_for_unknown_hyp_is_error_state()
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
pub fn hyp_becomes_part_of_parent()
{
    let mut session = new_started_session();

    let project_info = TestHypKind::Project(TestProject::new("example_project"));
    session.hyp(project_info);

    let hyp = TestHypKind::Hyp(TestHyp::new("example_project::example_hyp"));
    session.hyp(hyp);

    let project_id = chain!("example_project");
    let project = session.hyps().entry(project_id).unwrap();

    assert_equal(project.children().iter(), [&hyp]);
}

#[test]
pub fn nested_hyps_are_included_in_all_hyps()
{
    let mut session = new_started_session();

    let project = TestProject::new("test_project");
    session.hyp(project.clone());

    let hyp = TestHyp::new("test 1");
    session.hyp_node(hyp.clone());

    assert_equal(session.all_hyps(), [&project, &hyp]);
}

fn new_started_session() -> TestSession
{
    let mut session = TestSession::new();

    session.start_run();

    session
}
