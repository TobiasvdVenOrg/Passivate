use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::bridge_hyp::BridgeHyp;
use passivate_model_bridge::hyp_state::HypState;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hyp<TBridge: Bridge>
{
    info: TBridge::HypInfo,
    state: HypState,
    output: Vec<TBridge::Output>
}

impl<TBridge: Bridge> Hyp<TBridge>
{
    pub fn new(info: TBridge::HypInfo) -> Self
    {
        Self::with_state(info, HypState::Unknown)
    }

    pub fn with_state(info: TBridge::HypInfo, state: HypState) -> Self
    {
        Self {
            info,
            state,
            output: Vec::new()
        }
    }

    pub fn id(&self) -> &TBridge::Id
    {
        self.info.id()
    }

    pub fn name(&self) -> std::borrow::Cow<'_, str>
    {
        self.info.name()
    }

    pub fn info(&self) -> &TBridge::HypInfo
    {
        &self.info
    }

    pub fn has_output(&self) -> bool
    {
        !self.output.is_empty()
    }

    pub fn iter_output(&self) -> impl Iterator<Item = &TBridge::Output>
    {
        self.output.iter()
    }

    pub fn add_output(&mut self, output: TBridge::Output)
    {
        self.output.push(output);
    }

    pub fn state(&self) -> HypState
    {
        self.state
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
