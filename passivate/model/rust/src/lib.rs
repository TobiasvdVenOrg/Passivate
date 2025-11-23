use std::ops::Deref;

use camino::Utf8PathBuf;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::package_id::PackageId;
use passivate_model_core::bridge::{Bridge, ProjectId};
use passivate_model_core::hyp_session_event::{CompilationMessage, ProjectCompilation};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageInfo
{
    pub package_id: PackageId,
    pub manifest_path: Utf8PathBuf
}

impl ProjectId for PackageInfo
{
    type TId = PackageId;

    fn id(&self) -> &Self::TId
    {
        &self.package_id
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustHyp(HypId);

impl Deref for RustHyp
{
    type Target = HypId;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl ProjectId for RustHyp
{
    type TId = PackageId;

    fn id(&self) -> &Self::TId
    {
        self.package_id()
    }
}

impl Bridge for RustBridge
{
    type THypNode = RustHyp;
    type TProjectCompilation = ProjectCompilation<PackageId>;
    type TProjectId = PackageId;
    type TProjectInfo = PackageInfo;
    type TWorkspaceCompilation = WorkspaceCompilation;
}
