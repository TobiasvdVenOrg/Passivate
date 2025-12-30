use std::borrow::Cow;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::bridge_hyp::BridgeHyp;
use passivate_model_bridge::hyp_run_bridge::HypRunBridge;
use passivate_model_bridge::hyp_session_bridge::HypSessionBridge;
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_bridge::output_report::OutputReport;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_event::{CompilationMessage, HypSessionEvent};

#[derive(Debug, PartialEq, Eq)]
pub struct TestSession(HypSession<TestSession>);

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
pub struct TestId(Vec<String>);

impl TestId
{
    pub fn empty() -> Self
    {
        Self(Vec::new())
    }
}

impl Display for TestId
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.0.join("::"))
    }
}

impl<T> From<T> for TestId
where
    T: Into<String>
{
    fn from(value: T) -> Self
    {
        Self(value.into().split("::").map(String::from).collect())
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
pub enum TestHypKind
{
    Hyp(TestHyp),
    Project(TestProject)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestHyp
{
    id: TestId
}

impl TestHyp
{
    pub fn new(id: impl Into<TestId>) -> Self
    {
        Self { id: id.into() }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestProject
{
    id: TestId
}

impl TestProject
{
    pub fn new(id: impl Into<TestId>) -> Self
    {
        Self { id: id.into() }
    }
}

impl BridgeHyp for TestHypKind
{
    type Id = TestId;

    fn id(&self) -> &Self::Id
    {
        match self
        {
            TestHypKind::Hyp(test_hyp) => &test_hyp.id,
            TestHypKind::Project(test_project) => &test_project.id
        }
    }

    fn name(&self) -> Cow<'_, str>
    {
        Cow::Owned(self.id().to_string())
    }

    fn state(&self) -> Option<HypState>
    {
        Some(HypState::Unknown)
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

#[derive(Debug, Clone, Eq, PartialEq, Display)]
pub enum TestOutput
{
    Compilation(CompilationMessage)
}

impl<T> From<T> for TestOutput
where
    T: Into<String>
{
    fn from(value: T) -> Self
    {
        Self::Compilation(CompilationMessage::new_info(value))
    }
}

impl HypRunBridge for TestSession
{
    fn run_hyps(&self)
    {
        todo!()
    }
}

impl Bridge for TestSession
{
    type HypInfo = TestHypKind;
    type HypRunner = TestSession;
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

    fn send_output(&mut self, report: OutputReport<TestSession>)
    {
        self.0.update(HypSessionEvent::Output(report));
    }

    fn send_hyp(&mut self, hyp: TestHypKind)
    {
        self.0.update(HypSessionEvent::HypExists(hyp));
    }

    fn complete_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunCompleted);
    }
}
