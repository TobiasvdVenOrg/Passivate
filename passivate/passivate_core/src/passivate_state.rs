use passivate_delegation::Rx;
use passivate_hyp_model::{hyp_run_events::HypRunEvent, test_run::TestRun};
use passivate_hyp_names::hyp_id::HypId;

pub struct PassivateState
{
    pub persisted: PersistedPassivateState,
    hyp_run_rx: Rx<HypRunEvent>
}

impl PassivateState
{
    pub fn new(hyp_run_rx: Rx<HypRunEvent>) -> Self
    {
        Self {
            persisted: PersistedPassivateState::default(),
            hyp_run_rx
        }
    }

    pub fn with_persisted_state(persisted: PersistedPassivateState, hyp_run_rx: Rx<HypRunEvent>) -> Self
    {
        Self {
            persisted,
            hyp_run_rx
        }
    }

    pub fn update(&mut self)
    {
        if let Ok(hyp_run_event) = self.hyp_run_rx.try_recv()
        {
            self.persisted.hyp_run.update(hyp_run_event);
        }
    }
}

#[derive(Default)]
pub struct PersistedPassivateState
{
    pub hyp_run: TestRun,
    pub selected_hyp: Option<HypId>
}
