use super::PassivateConfig;


pub struct ConfigurationEvent {
    pub old: PassivateConfig,
    pub new: PassivateConfig
}

#[derive(Clone)]
pub enum ConfigurationChangeEvent {
    Coverage(bool)
}
