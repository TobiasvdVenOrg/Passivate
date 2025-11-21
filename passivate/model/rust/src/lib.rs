use camino::Utf8PathBuf;
use passivate_model_core::bridge::Bridge;
use passivate_model_core::hyp_session_event::CompilationMessage;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HypPackage
{
    pub package_name: String,
    pub manifest_path: Utf8PathBuf
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
    type TWorkspaceCompilation = WorkspaceCompilation;
}
