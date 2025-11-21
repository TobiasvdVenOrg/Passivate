use passivate_hyp_names::hyp_id::HypId;

use crate::bridge::Bridge;

#[derive(Debug, Clone)]
pub enum ProjectCompilationEventKind
{
    StartCompilation,
    Message(CompilationMessage)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectCompilationEvent {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationMessageKind
{
    Info,
    Warning,
    Error
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationMessage
{
    pub content: String,
    pub kind: CompilationMessageKind
}

impl CompilationMessage
{
    pub fn new_info(message: impl Into<String>) -> Self
    {
        Self {
            content: message.into(),
            kind: CompilationMessageKind::Info
        }
    }

    pub fn new_warning(message: impl Into<String>) -> Self
    {
        Self {
            content: message.into(),
            kind: CompilationMessageKind::Warning
        }
    }

    pub fn new_error(message: impl Into<String>) -> Self
    {
        Self {
            content: message.into(),
            kind: CompilationMessageKind::Error
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypSessionEvent<TBridge: Bridge>
{
    RunStarted,
    ProjectExists(TBridge::TProject),
    WorkspaceCompilation(TBridge::TWorkspaceCompilation),
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
