use super::PassivateConfig;

#[derive(Clone, PartialEq, Debug)]
pub struct ConfigurationEvent {
    pub old: PassivateConfig,
    pub new: PassivateConfig
}
