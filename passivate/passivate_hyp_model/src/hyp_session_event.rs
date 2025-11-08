use camino::Utf8PathBuf;
use passivate_hyp_names::hyp_id::HypId;

#[derive(Debug, Clone)]
pub enum CrateCompilationEvent
{
    StartCompilation
    {
        crate_name: String,
        version: String,
        path: Utf8PathBuf
    },
    Message(CompilationMessage)
}

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

pub struct HypCrate {}

#[derive(Debug, Clone)]
pub enum HypSessionEvent
{
    RunStarted,
    WorkspaceCompilation(WorkspaceCompilationEvent),
    CrateExists,
    CrateCompilation(CrateCompilationEvent),
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
