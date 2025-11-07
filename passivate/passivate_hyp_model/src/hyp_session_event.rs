use camino::Utf8PathBuf;
use passivate_hyp_names::hyp_id::HypId;

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

pub enum CompilationMessageKind
{
    Warning,
    Error
}

pub struct CompilationMessage
{
    pub content: String,
    pub kind: CompilationMessageKind
}

pub enum WorkspaceCompilationEvent
{
    WaitForLock,
    Message(CompilationMessage)
}

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
