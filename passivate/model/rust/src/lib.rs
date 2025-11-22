use camino::Utf8PathBuf;
use passivate_model_core::bridge::{Bridge, ProjectId};
use passivate_model_core::hyp_session_event::{CompilationMessage, ProjectCompilation};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HypPackage
{
    pub package_name: String,
    pub manifest_path: Utf8PathBuf
}

impl ProjectId for HypPackage
{
    type T = String;

    fn id(&self) -> &Self::T
    {
        &self.package_name
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WorkspaceCompilation
{
    WaitForLock,
    Message(CompilationMessage)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustBridge;

impl Bridge for RustBridge
{
    type TProject = HypPackage;
    type TProjectCompilation = ProjectCompilation<String>;
    type TProjectId = String;
    type TWorkspaceCompilation = WorkspaceCompilation;
}
