use std::fmt::Debug;

use passivate_id_chain_tree::tree::Tree;
use passivate_model_bridge::{Bridge, OutputReport};

use crate::evaluate::Evaluate;
use crate::hyp::Hyp;
use crate::hyp_session_change::HypSessionChange;
use crate::hyp_session_event::HypSessionEvent;
use crate::hyp_session_state_error::HypSessionStateError;
use crate::hyp_state::HypState;

#[derive(Debug, PartialEq, Eq)]
pub struct HypSession<TBridge: Bridge>
{
    session: Session<TBridge>,
    error: Option<HypSessionStateError<TBridge>>
}

#[derive(Debug, PartialEq, Eq)]
struct Session<TBridge: Bridge>
{
    activity: HypState,
    hyps: Tree<TBridge::IdLink, Hyp<TBridge>>
}

type ChangeResult<TBridge> = Result<Option<HypSessionChange<TBridge>>, HypSessionEvent<TBridge>>;

impl<TBridge: Bridge> HypSession<TBridge>
{
    pub fn new() -> Self
    {
        let session = Session {
            activity: HypState::Unknown,
            hyps: Tree::new()
        };

        HypSession { session, error: None }
    }

    pub fn from_events(events: impl IntoIterator<Item = HypSessionEvent<TBridge>>) -> Self
    {
        let mut session = Self::new();
        session.update_all(events);
        session
    }

    pub fn activity(&self) -> Result<HypState, &HypSessionStateError<TBridge>>
    {
        self.error.as_ref().map_or_else(|| Ok(self.session.activity), |e| Err(e))
    }

    pub fn hyps(&self) -> &Tree<TBridge::IdLink, Hyp<TBridge>>
    {
        &self.session.hyps
    }

    pub fn update_all(&mut self, events: impl IntoIterator<Item = HypSessionEvent<TBridge>>)
    {
        for event in events
        {
            self.update(event);
        }
    }

    pub fn update(&mut self, event: HypSessionEvent<TBridge>) -> Option<HypSessionChange<TBridge>>
    {
        if self.error.is_some()
        {
            return None;
        }

        match self.session.update(event)
        {
            Ok(change) => change,
            Err(error) =>
            {
                self.error = Some(error);
                None
            }
        }
    }
}

impl<TBridge: Bridge> Evaluate for HypSession<TBridge>
{
    fn state(&self) -> HypState
    {
        self.activity().unwrap_or(HypState::Failed)
    }
}

impl<TBridge: Bridge> Session<TBridge>
{
    fn update(
        &mut self,
        event: HypSessionEvent<TBridge>
    ) -> Result<Option<HypSessionChange<TBridge>>, HypSessionStateError<TBridge>>
    {
        self.process_event(event)
            .map_err(|error_event| HypSessionStateError::UnexpectedEvent { event: error_event })
    }

    fn process_event(&mut self, event: HypSessionEvent<TBridge>) -> ChangeResult<TBridge>
    {
        match self.activity
        {
            HypState::Unknown | HypState::Passed | HypState::Failed =>
            {
                match event
                {
                    HypSessionEvent::RunStarted => self.start_run(),
                    _ => Err(event)
                }
            }
            HypState::Running =>
            {
                match event
                {
                    HypSessionEvent::Output(report) => self.output(report),
                    HypSessionEvent::HypExists(project) => self.hyp_exists(project),
                    HypSessionEvent::RunCompleted => self.complete_run(),
                    _ => Err(event)
                }
            }
        }
    }

    fn start_run(&mut self) -> ChangeResult<TBridge>
    {
        self.activity = HypState::Running;

        Ok(None)
    }

    fn complete_run(&mut self) -> ChangeResult<TBridge>
    {
        self.activity = HypState::Passed;

        Ok(None)
    }

    fn output(&mut self, output: OutputReport<TBridge>) -> ChangeResult<TBridge>
    {
        todo!()
    }

    fn hyp_exists(&mut self, hyp_node_info: TBridge::HypInfo) -> ChangeResult<TBridge>
    {
        let hyp: Hyp<TBridge> = Hyp::new(hyp_node_info);

        self.hyps.insert(hyp);

        Ok(None)
    }
}
