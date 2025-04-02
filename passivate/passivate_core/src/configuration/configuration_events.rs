use super::PassivateConfig;


pub struct ConfigurationEvent {
    pub old: Option<PassivateConfig>,
    pub new: PassivateConfig
}

#[derive(Clone)]
pub enum ConfigurationChangeEvent {
    Coverage(bool),
    SnapshotsPath(String)
}
