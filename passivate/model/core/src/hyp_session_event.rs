use passivate_hyp_names::hyp_id::HypId;

use crate::bridge::Bridge;

#[derive(Debug, Clone)]
pub enum ProjectCompilationEventKind
{
    StartCompilation,
    Message(CompilationMessage)
}

#[derive(Debug, Clone)]
pub struct ProjectCompilationEvent {}

#[derive(Debug, Clone)]
pub enum CompilationMessageKind
{
    Warning,
    Error
}

#[derive(Debug, Clone)]
pub struct CompilationMessage
{
    pub content: String,
    pub kind: CompilationMessageKind
}

#[derive(Debug, Clone)]
pub enum WorkspaceCompilationEvent
{
    WaitForLock,
    Message(CompilationMessage)
}

#[derive(Debug, Clone)]
pub enum HypSessionEvent<TBridge: Bridge>
{
    RunStarted,
    ProjectExists(TBridge::TProject),
    WorkspaceCompilation(WorkspaceCompilationEvent),
    ProjectCompilation(ProjectCompilationEvent),
    HypExists(HypId),
    HypRunning(HypId),
    HypStdOut
    {
        id: HypId,
        lines: Vec<String>
    },
    HypStdErr
    {
        id: HypId,
        lines: Vec<String>
    },
    HypCompleted(HypId),
    RunCompleted
}
