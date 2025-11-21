use passivate_delegation::Rx;
use passivate_hyp_names::hyp_id::HypId;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_change::HypSessionChange;
use passivate_model_core::hyp_session_event::HypSessionEvent;
use passivate_model_rust::RustBridge;

use crate::passivate_state_change::PassivateStateChange;

pub struct PassivateState
{
    pub hyp_session: HypSession<RustBridge>,
    pub selected_hyp: Option<HypId>,
    hyp_run_rx: Rx<HypSessionEvent<RustBridge>>
}

impl PassivateState
{
    pub fn new(hyp_run_rx: Rx<HypSessionEvent<RustBridge>>) -> Self
    {
        Self::with_initial_session_state(HypSession::new(), hyp_run_rx)
    }

    pub fn with_initial_session_state(hyp_session: HypSession<RustBridge>, hyp_run_rx: Rx<HypSessionEvent<RustBridge>>)
    -> Self
    {
        Self {
            hyp_session,
            selected_hyp: None,
            hyp_run_rx
        }
    }

    pub fn update(&mut self) -> Option<PassivateStateChange<'_>>
    {
        self.try_process_hyp_run_event().map(Self::map_hyp_run_change).flatten()
    }

    fn map_hyp_run_change(change: HypSessionChange<'_, RustBridge>) -> Option<PassivateStateChange<'_>>
    {
        match change
        {
            HypSessionChange::HypUpdated(single_hyp) => Some(PassivateStateChange::HypDetailsChanged(single_hyp)),
            HypSessionChange::NewProject(_) => None
        }
    }

    fn try_process_hyp_run_event(&mut self) -> Option<HypSessionChange<'_, RustBridge>>
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

    fn process_hyp_run_event(&mut self, event: HypSessionEvent<RustBridge>) -> Option<HypSessionChange<'_, RustBridge>>
    {
        self.hyp_session.update(event)
    }
}
