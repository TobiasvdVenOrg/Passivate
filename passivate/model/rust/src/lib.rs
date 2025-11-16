use camino::Utf8PathBuf;
use passivate_model_core::bridge::{Bridge, ProjectTrait};

#[derive(Debug, Clone)]
pub struct HypPackage
{
    pub package_name: String,
    pub manifest_path: Utf8PathBuf
}

impl ProjectTrait for HypPackage {}

#[derive(Debug, Clone)]
pub struct RustBridge;

impl Bridge for RustBridge
{
    type TProject = HypPackage;
}
