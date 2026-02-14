use passivate_configuration::configuration::PassivateConfiguration;
use passivate_configuration::default_paths::{self, DefaultPaths};

use crate::bridge::Bridge;

#[derive(Debug)]
pub struct HypRunRequest<TBridge: Bridge>
{
    pub kind: HypRunRequestKind<TBridge>,
    pub configuration: PassivateConfiguration,
    pub paths: DefaultPaths
}

impl<TBridge: Bridge> HypRunRequest<TBridge>
{
    pub fn all(configuration: PassivateConfiguration, paths: DefaultPaths) -> Self
    {
        Self {
            kind: HypRunRequestKind::All,
            configuration,
            paths
        }
    }

    pub fn single(hyp_id: TBridge::Id, configuration: PassivateConfiguration, paths: DefaultPaths) -> Self
    {
        Self {
            kind: HypRunRequestKind::Single { hyp_id },
            configuration,
            paths
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum HypRunRequestKind<TBridge: Bridge>
{
    All,
    Single
    {
        hyp_id: TBridge::Id
    }
}

#[cfg(feature = "testing")]
#[bon::bon]
impl<TBridge: Bridge> HypRunRequest<TBridge>
{
    #[builder]
    pub fn stub(
        #[builder(default = HypRunRequestKind::All)] kind: HypRunRequestKind<TBridge>,
        #[builder(default = PassivateConfiguration::default())] configuration: PassivateConfiguration,
        #[builder(default = default_paths::stub())] paths: DefaultPaths
    ) -> HypRunRequest<TBridge>
    {
        HypRunRequest {
            kind,
            configuration,
            paths
        }
    }
}
