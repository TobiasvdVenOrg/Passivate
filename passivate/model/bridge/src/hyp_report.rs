use crate::bridge::Bridge;
use crate::hyp_state::HypState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypReportState
{
    Fixed(HypState),
    Derived
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HypReport<TBridge: Bridge>
{
    pub hyp_info: TBridge::HypInfo,
    pub state: HypReportState
}

impl<TBridge: Bridge> HypReport<TBridge>
{
    pub fn new_fixed(hyp_info: TBridge::HypInfo, state: HypState) -> Self
    {
        Self {
            hyp_info,
            state: HypReportState::Fixed(state)
        }
    }

    pub fn new_derived(hyp_info: TBridge::HypInfo) -> Self
    {
        Self {
            hyp_info,
            state: HypReportState::Derived
        }
    }
}
