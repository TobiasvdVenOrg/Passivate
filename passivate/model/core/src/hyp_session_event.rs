use passivate_hyp_names::hyp_id::HypId;

use crate::bridge::{Bridge, ProjectId};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProjectCompilation<TProjectId>
{
    pub project_id: TProjectId,
    pub message: CompilationMessage
}

impl<TProjectId> ProjectId for ProjectCompilation<TProjectId>
{
    type TId = TProjectId;

    fn id(&self) -> &Self::TId
    {
        &self.project_id
    }
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
    ProjectExists(TBridge::TProjectInfo),
    WorkspaceCompilation(TBridge::TWorkspaceCompilation),
    ProjectCompilation(TBridge::TProjectCompilation),
    HypNodeExists(TBridge::THypNode),
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
