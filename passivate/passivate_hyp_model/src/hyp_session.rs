use crate::hyp_run_events::HypRunChange;
use crate::hyp_session_event::HypSessionEvent;
use crate::hyp_session_state::{HypSessionState, HypSessionStateError};

#[derive(Debug, Clone)]
pub struct HypSession
{
    state: Result<HypSessionState, HypSessionStateError>
}

impl HypSession
{
    pub fn new() -> Self
    {
        Self {
            state: Ok(HypSessionState::Idle)
        }
    }

    pub fn from_events(events: impl IntoIterator<Item = HypSessionEvent>) -> Self
    {
        let mut session = Self::new();
        session.update_all(events);
        session
    }

    pub fn state(&self) -> Result<&HypSessionState, &HypSessionStateError>
    {
        self.state.as_ref()
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

        self.state = match &self.state
        {
            Ok(current_state) => Self::process_state(event, current_state),
            Err(error_state) => todo!()
        };

        change
    }

    fn process_state(event: HypSessionEvent, current_state: &HypSessionState) -> Result<HypSessionState, HypSessionStateError>
    {
        match event
        {
            HypSessionEvent::RunStarted => Self::start_run(current_state),
            HypSessionEvent::RunCompleted => Self::complete_run(current_state),
            _ => todo!()
        }
    }

    fn start_run(current_state: &HypSessionState) -> Result<HypSessionState, HypSessionStateError>
    {
        Self::try_transition(current_state, HypSessionState::Idle, HypSessionState::Running)
    }

    fn complete_run(current_state: &HypSessionState) -> Result<HypSessionState, HypSessionStateError>
    {
        Self::try_transition(current_state, HypSessionState::Running, HypSessionState::Idle)
    }

    fn try_transition(
        current_state: &HypSessionState,
        expected_state: HypSessionState,
        new_state: HypSessionState
    ) -> Result<HypSessionState, HypSessionStateError>
    {
        if *current_state == expected_state
        {
            Ok(new_state)
        }
        else
        {
            Err(HypSessionStateError::UnexpectedStateChange {
                from: current_state.clone(),
                to: new_state
            })
        }
    }
}
