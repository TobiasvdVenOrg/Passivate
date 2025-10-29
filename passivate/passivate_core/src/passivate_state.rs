use passivate_delegation::Rx;
use passivate_hyp_model::hyp_run_events::{HypRunChange, HypRunEvent};
use passivate_hyp_model::hyp_run::HypRun;
use passivate_hyp_names::hyp_id::HypId;

use crate::passivate_state_change::PassivateStateChange;

pub struct PassivateState
{
    pub hyp_run: HypRun,
    pub selected_hyp: Option<HypId>,
    hyp_run_rx: Rx<HypRunEvent>
}

impl PassivateState
{
    pub fn new(hyp_run_rx: Rx<HypRunEvent>) -> Self
    {
        Self::with_initial_run_state(HypRun::default(), hyp_run_rx)
    }

    pub fn with_initial_run_state(hyp_run: HypRun, hyp_run_rx: Rx<HypRunEvent>) -> Self
    {
        Self {
            hyp_run,
            selected_hyp: None,
            hyp_run_rx
        }
    }

    pub fn update(&mut self) -> Option<PassivateStateChange<'_>>
    {
        self.try_process_hyp_run_event().map(Self::map_hyp_run_change)
    }

    fn map_hyp_run_change(change: HypRunChange<'_>) -> PassivateStateChange<'_>
    {
        match change
        {
            HypRunChange::HypUpdated(single_hyp) => PassivateStateChange::HypDetailsChanged(single_hyp),
        }
    }

    fn try_process_hyp_run_event(&mut self) -> Option<HypRunChange<'_>>
    {
        if let Ok(hyp_run_event) = self.hyp_run_rx.try_recv()
        {
            self.process_hyp_run_event(hyp_run_event)
        }
        else 
        {
            None
        }
    }

    fn process_hyp_run_event(&mut self, event: HypRunEvent) -> Option<HypRunChange<'_>>
    {
        self.hyp_run.update(event)
    }
}
