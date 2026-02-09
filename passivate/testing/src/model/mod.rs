use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::bridge_hyp::BridgeHyp;
use passivate_model_bridge::hyp_report::HypReport;
use passivate_model_bridge::hyp_session_bridge::{
    CancelRunBridge,
    CompleteRunBridge,
    RunErrorBridge,
    SendHypBridge,
    SendOutputBridge,
    StartRunBridge
};
use passivate_model_bridge::hyp_session_event::{CompilationMessage, HypSessionEvent};
use passivate_model_bridge::output_report::OutputReport;
use passivate_model_core::hyp_session::HypSession;

#[derive(Default, Debug, PartialEq, Eq)]
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

impl From<TestSession> for HypSession<TestSession>
{
    fn from(val: TestSession) -> Self
    {
        val.0
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
}

impl Display for TestHypKind
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.id())
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TestOutput
{
    Compilation(CompilationMessage)
}

impl Display for TestOutput
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        todo!()
    }
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

impl Bridge for TestSession
{
    type HypInfo = TestHypKind;
    type Id = TestId;
    type IdLink = String;
    type Output = TestOutput;
    type RunError = String;
}

impl StartRunBridge<TestSession> for TestSession
{
    fn start_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunStarted);
    }
}

impl SendOutputBridge<TestSession> for TestSession
{
    fn send_output(&mut self, report: OutputReport<TestSession>)
    {
        self.0.update(HypSessionEvent::Output(report));
    }
}

impl SendHypBridge<TestSession> for TestSession
{
    fn send_hyp(&mut self, hyp_report: HypReport<TestSession>)
    {
        self.0.update(HypSessionEvent::Hyp(hyp_report));
    }
}

impl CompleteRunBridge<TestSession> for TestSession
{
    fn complete_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunCompleted);
    }
}

impl CancelRunBridge<TestSession> for TestSession
{
    fn cancel_run(&mut self)
    {
        self.0.update(HypSessionEvent::RunCancelled);
    }
}

impl RunErrorBridge<TestSession> for TestSession
{
    fn run_error(&mut self, run_error: <TestSession as Bridge>::RunError)
    {
        self.0.update(HypSessionEvent::RunError(run_error));
    }
}
