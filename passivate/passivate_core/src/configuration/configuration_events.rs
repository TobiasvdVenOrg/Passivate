use super::PassivateConfig;

#[derive(Clone)]
pub enum ConfigurationEvent {
    Update(PassivateConfig),
    Coverage(bool)
}
