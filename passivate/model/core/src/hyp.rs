use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::Bridge;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hyp<TBridge: Bridge>
{
    pub info: TBridge::HypInfo,
    pub output: Vec<TBridge::Output>
}

impl<TBridge: Bridge> Hyp<TBridge>
{
    pub fn new(info: TBridge::HypInfo) -> Self
    {
        Self {
            info,
            output: Vec::new()
        }
    }
}

impl<TBridge: Bridge> IdChain for Hyp<TBridge>
{
    type Link = TBridge::IdLink;

    fn chain(&self) -> &[Self::Link]
    {
        self.info.chain()
    }
}
