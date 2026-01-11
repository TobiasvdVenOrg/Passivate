use passivate_coverage::coverage_status::CoverageStatus;
use passivate_model_bridge::bridge::Bridge;

use crate::passivate_state_change::PassivateStateChange;

pub struct PassivateState<TBridge: Bridge>
{
    pub selected_hyp: Option<TBridge::Id>,
    pub coverage: CoverageStatus
}

impl<TBridge: Bridge> PassivateState<TBridge>
{
    pub fn new() -> Self
    {
        Self {
            selected_hyp: None,
            coverage: CoverageStatus::Disabled
        }
    }

    pub fn update_state(&mut self, change: &PassivateStateChange<'_, TBridge>) {}
}
