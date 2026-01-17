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
        Self::default()
    }

    pub fn update_state(&mut self, change: Option<&PassivateStateChange<'_, TBridge>>)
    {
        if let Some(change) = change
        {
            self.process_change(change);
        }
    }

    fn process_change(&mut self, change: &PassivateStateChange<'_, TBridge>)
    {
        match change
        {
            PassivateStateChange::HypSelected(hyp) => self.selected_hyp = Some(hyp.id().clone()),
            PassivateStateChange::HypDetailsChanged(_hyp) => todo!(),
            PassivateStateChange::ConfigurationChanged(_configuration_change) => todo!()
        }
    }
}

impl<TBridge: Bridge> Default for PassivateState<TBridge>
{
    fn default() -> Self
    {
        Self {
            selected_hyp: None,
            coverage: CoverageStatus::Disabled
        }
    }
}
