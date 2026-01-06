use std::borrow::Cow;
use std::fmt::{Display, Pointer};

use camino::Utf8PathBuf;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::hyp_name_strategy::HypNameStrategy;
use passivate_hyp_names::package_id::PackageId;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::bridge_hyp::BridgeHyp;
use passivate_model_bridge::hyp_session_event::{CompilationMessage, ConsoleOutput};
use passivate_model_bridge::hyp_state::HypState;

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
    id: HypId,
    pub kind: RustHypKind
}

impl RustHyp
{
    pub fn new_single(id: HypId) -> Self
    {
        Self {
            id,
            kind: RustHypKind::Single(SingleRustHyp {
                state: HypState::Unknown,
                name_strategy: HypNameStrategy::Default
            })
        }
    }

    pub fn new_package(id: HypId, info: PackageInfo) -> Self
    {
        Self {
            id,
            kind: RustHypKind::Package(info)
        }
    }

    pub fn name(&self) -> String
    {
        match &self.kind
        {
            RustHypKind::Single(single_rust_hyp) => self.id.name(&single_rust_hyp.name_strategy).to_string(),
            RustHypKind::Package(package_info) => package_info.package_id.to_string()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleRustHyp
{
    state: HypState,
    name_strategy: HypNameStrategy
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RustHypKind
{
    Single(SingleRustHyp),
    Package(PackageInfo)
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

impl BridgeHyp for RustHyp
{
    type Id = HypId;

    fn id(&self) -> &Self::Id
    {
        &self.id
    }

    fn name(&self) -> Cow<'_, str>
    {
        self.id.name(&HypNameStrategy::Default)
    }

    fn state(&self) -> Option<HypState>
    {
        match &self.kind
        {
            RustHypKind::Single(hyp) => Some(hyp.state),
            RustHypKind::Package(_) => None
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
    type Id = HypId;
    type IdLink = String;
    type Output = RustOutput;
}
