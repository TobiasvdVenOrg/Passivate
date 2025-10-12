use crate::configuration::Configuration;

#[derive(Clone, PartialEq, Debug)]
pub struct ConfigurationEvent
{
    pub old: Configuration,
    pub new: Configuration
}
