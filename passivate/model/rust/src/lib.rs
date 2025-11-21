use camino::Utf8PathBuf;
use passivate_model_core::bridge::Bridge;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HypPackage
{
    pub package_name: String,
    pub manifest_path: Utf8PathBuf
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustBridge;

impl Bridge for RustBridge
{
    type TProject = HypPackage;
}
