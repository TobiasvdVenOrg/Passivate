use super::PassivateConfig;


pub enum ConfigurationEvent {
    Update(PassivateConfig),
    Coverage(bool)
}
