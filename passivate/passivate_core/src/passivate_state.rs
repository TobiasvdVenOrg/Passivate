use passivate_delegation::Rx;
use passivate_hyp_model::hyp_session::HypSession;
use passivate_hyp_model::hyp_session_change::{HypSessionChange, HypSessionEvent};
use passivate_hyp_names::hyp_id::HypId;

use crate::passivate_state_change::PassivateStateChange;

pub struct PassivateState
{
    pub hyp_session: HypSession,
    pub selected_hyp: Option<HypId>,
    hyp_run_rx: Rx<HypSessionEvent>
}

impl PassivateState
{
    pub fn new(hyp_run_rx: Rx<HypSessionEvent>) -> Self
    {
        Self::with_initial_session_state(HypSession::default(), hyp_run_rx)
    }

    pub fn with_initial_session_state(hyp_session: HypSession, hyp_run_rx: Rx<HypSessionEvent>) -> Self
    {
        Self {
            hyp_session,
            selected_hyp: None,
            hyp_run_rx
        }
    }

    pub fn update(&mut self) -> Option<PassivateStateChange<'_>>
    {
        self.try_process_hyp_run_event().map(Self::map_hyp_run_change)
    }

    fn map_hyp_run_change(change: HypSessionChange<'_>) -> PassivateStateChange<'_>
    {
        match change
        {
            HypSessionChange::HypUpdated(single_hyp) => PassivateStateChange::HypDetailsChanged(single_hyp)
        }
    }

    fn try_process_hyp_run_event(&mut self) -> Option<HypSessionChange<'_>>
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

    fn process_hyp_run_event(&mut self, event: HypSessionEvent) -> Option<HypSessionChange<'_>>
    {
        self.hyp_session.update(event)
    }
}
