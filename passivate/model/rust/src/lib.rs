use std::fmt::{Display, Pointer};

use camino::Utf8PathBuf;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::package_id::PackageId;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::Bridge;
use passivate_model_core::hyp_session_event::{CompilationMessage, ConsoleOutput};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageInfo
{
    pub package_id: PackageId,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustHyp
{
    pub id: HypId
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustOutput
{
    Workspace(WorkspaceCompilation),
    Project(CompilationMessage),
    Console(ConsoleOutput)
}

impl Display for RustOutput
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            RustOutput::Workspace(workspace_compilation) => workspace_compilation.fmt(f),
            RustOutput::Project(compilation_message) => compilation_message.fmt(f),
            RustOutput::Console(console_output) => console_output.fmt(f)
        }
    }
}

impl IdChain for RustHyp
{
    type Link = String;

    fn chain(&self) -> &[Self::Link]
    {
        self.id.chain()
    }
}

impl Bridge for RustBridge
{
    type HypInfo = RustHyp;
    type Id = String;
    type Output = RustOutput;
}
