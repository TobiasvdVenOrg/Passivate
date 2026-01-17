use passivate_configuration::configuration::ConfigurationChange;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_core::hyp::Hyp;

#[derive(Debug)]
pub enum PassivateStateChange<'a, TBridge: Bridge>
{
    HypSelected(&'a Hyp<TBridge>),
    HypDetailsChanged(&'a Hyp<TBridge>),
    ConfigurationChanged(ConfigurationChange)
}

impl<'a, TBridge: Bridge> PassivateStateChange<'a, TBridge>
{
    pub fn requires_rerun(&self) -> bool
    {
        match self
        {
            PassivateStateChange::HypSelected(_) => false,
            PassivateStateChange::HypDetailsChanged(_) => false,
            PassivateStateChange::ConfigurationChanged(_) => true
        }
    }
}
