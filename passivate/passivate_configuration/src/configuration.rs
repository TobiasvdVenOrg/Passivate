use camino::Utf8PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct PassivateConfiguration
{
    pub coverage_enabled: bool,
    pub snapshots_path: Option<Utf8PathBuf>
}
