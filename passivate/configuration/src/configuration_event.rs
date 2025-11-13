use crate::configuration::PassivateConfiguration;

#[derive(Clone, PartialEq, Debug)]
pub struct ConfigurationEvent
{
    pub old: PassivateConfiguration,
    pub new: PassivateConfiguration
}
