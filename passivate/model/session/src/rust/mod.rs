use camino::Utf8PathBuf;

use crate::hyp_session_event::{HypProject, HypProjectKind};

#[derive(Debug, Clone)]
pub struct RustHypProject
{
    pub package_name: String,
    pub manifest_path: Utf8PathBuf
}

impl From<RustHypProject> for HypProject
{
    fn from(value: RustHypProject) -> Self
    {
        HypProject::new(HypProjectKind::Rust(value))
    }
}
