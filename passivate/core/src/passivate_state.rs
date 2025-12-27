use passivate_delegation::Rx;
use passivate_hyp_names::hyp_id::HypId;
use passivate_model_bridge::Bridge;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_change::HypSessionChange;
use passivate_model_core::hyp_session_event::HypSessionEvent;

use crate::passivate_state_change::PassivateStateChange;

pub struct PassivateState<TBridge: Bridge>
{
    pub hyp_session: HypSession<TBridge>,
    pub selected_hyp: Option<HypId>,
    hyp_run_rx: Rx<HypSessionEvent<TBridge>>
}

impl<TBridge: Bridge> PassivateState<TBridge>
{
    pub fn new(hyp_run_rx: Rx<HypSessionEvent<TBridge>>) -> Self
    {
        Self::with_initial_session_state(HypSession::new(), hyp_run_rx)
    }

    pub fn with_initial_session_state(hyp_session: HypSession<TBridge>, hyp_run_rx: Rx<HypSessionEvent<TBridge>>) -> Self
    {
        Self {
            hyp_session,
            selected_hyp: None,
            hyp_run_rx
        }
    }

    pub fn update(&mut self) -> Option<PassivateStateChange<TBridge>>
    {
        self.try_process_hyp_run_event().map(Self::map_hyp_run_change).flatten()
    }

    fn map_hyp_run_change(change: HypSessionChange<TBridge>) -> Option<PassivateStateChange<TBridge>>
    {
        match change
        {
            HypSessionChange::HypUpdated(single_hyp) => Some(PassivateStateChange::HypDetailsChanged(single_hyp)),
            HypSessionChange::NewHyp(_) => None
        }
    }

    fn try_process_hyp_run_event(&mut self) -> Option<HypSessionChange<TBridge>>
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

    fn process_hyp_run_event(&mut self, event: HypSessionEvent<TBridge>) -> Option<HypSessionChange<TBridge>>
    {
        self.hyp_session.update(event)
    }
}
