use passivate_configuration::configuration::Configuration;


#[derive(Clone, PartialEq, Debug)]
pub struct ConfigurationEvent
{
    pub old: Configuration,
    pub new: Configuration
}
