use camino::{Utf8Path, Utf8PathBuf};
use passivate_hyp_names::hyp_id::HypId;

use crate::rust::RustHypProject;

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
pub struct HypProject
{
    kind: HypProjectKind
}

impl HypProject
{
    pub fn new(kind: HypProjectKind) -> Self
    {
        Self { kind }
    }
}

#[derive(Debug, Clone)]
pub enum HypProjectKind
{
    Rust(RustHypProject)
}

impl HypProject
{
    pub fn name(&self) -> &str
    {
        match &self.kind
        {
            HypProjectKind::Rust(project) => &project.package_name
        }
    }

    pub fn path(&self) -> &Utf8Path
    {
        match &self.kind
        {
            HypProjectKind::Rust(project) => &project.manifest_path
        }
    }
}

impl<T> From<T> for HypSessionEvent
where
    T: Into<HypProject>
{
    fn from(value: T) -> Self
    {
        HypSessionEvent::ProjectExists(value.into())
    }
}

#[derive(Debug, Clone)]
pub enum HypSessionEvent
{
    RunStarted,
    ProjectExists(HypProject),
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
