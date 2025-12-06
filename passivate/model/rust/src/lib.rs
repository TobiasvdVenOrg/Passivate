use std::ops::Deref;

use camino::Utf8PathBuf;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::package_id::PackageId;
use passivate_model_core::bridge::{Bridge, HypPath};
use passivate_model_core::hyp_session_event::CompilationMessage;
use radix_trie::TrieKey;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageInfo
{
    pub package_id: PackageId,
    pub manifest_path: Utf8PathBuf
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageKey(PackageId);

impl HypPath for PackageInfo
{
    type TId = Vec<u8>;

    fn path(&self) -> &Self::TId
    {
        self.package_id.as_bytes().to_vec()
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

impl HypPath for RustHyp
{
    type TId = String;

    fn path(&self) -> &Self::TId
    {
        &self.fully_qualified("::")
    }
}

impl Bridge for RustBridge
{
    type THypNodeInfo = RustHyp;
    type TId = String;
    type TOutput = CompilationMessage;
}
