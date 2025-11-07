use crate::hyp_run_events::HypRunChange;
use crate::hyp_session_event::HypSessionEvent;
use crate::hyp_session_state::HypSessionState;

#[derive(Debug, Clone, Default)]
pub struct HypSession
{
    state: HypSessionState
}

impl HypSession
{
    pub fn from_events(events: impl IntoIterator<Item = HypSessionEvent>) -> Self
    {
        let mut session = Self::default();
        session.update_all(events);
        session
    }

    pub fn state(&self) -> &HypSessionState
    {
        &self.state
    }

    pub fn no_hyps(&self) -> bool
    {
        true
    }

    pub fn update_all(&mut self, events: impl IntoIterator<Item = HypSessionEvent>)
    {
        for event in events
        {
            self.update(event);
        }
    }

    pub fn update(&mut self, event: HypSessionEvent) -> Option<HypRunChange<'_>>
    {
        let mut change = None;

        match event
        {
            HypSessionEvent::RunStarted => self.start_run(),
            _ => todo!()
        }

        change
    }

    fn start_run(&mut self)
    {
        self.state = HypSessionState::Running;
    }
}
