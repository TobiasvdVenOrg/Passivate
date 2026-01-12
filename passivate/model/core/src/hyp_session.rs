use std::fmt::Debug;

use passivate_delegation::Rx;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_id_chain_tree::tree::Tree;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_bridge::output_report::OutputReport;

use crate::hyp::Hyp;
use crate::hyp_session_change::HypSessionChange;
use crate::hyp_session_state_error::HypSessionStateError;

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
    hyps: Tree<TBridge::IdLink, Hyp<TBridge>>,
    output: Vec<TBridge::Output>
}

type ChangeResult<'a, TBridge> = Result<Option<HypSessionChange<'a, TBridge>>, HypSessionEvent<TBridge>>;

impl<TBridge: Bridge> HypSession<TBridge>
{
    pub fn new() -> Self
    {
        let session = Session {
            activity: HypState::Unknown,
            hyps: Tree::new(),
            output: Vec::new()
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

    pub fn iter_output(&self) -> impl Iterator<Item = &TBridge::Output>
    {
        self.session.output.iter()
    }

    pub fn update_all(&mut self, events: impl IntoIterator<Item = HypSessionEvent<TBridge>>)
    {
        for event in events
        {
            self.update(event);
        }
    }

    pub fn update_next(&mut self, events_rx: &impl Rx<HypSessionEvent<TBridge>>) -> Option<HypSessionChange<'_, TBridge>>
    {
        events_rx.try_recv().map(|event| self.update(event)).ok().flatten()
    }

    pub fn update(&mut self, event: HypSessionEvent<TBridge>) -> Option<HypSessionChange<'_, TBridge>>
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

    pub fn state(&self) -> HypState
    {
        self.activity().unwrap_or(HypState::Failed)
    }
}

impl<TBridge: Bridge> Session<TBridge>
{
    fn update(
        &mut self,
        event: HypSessionEvent<TBridge>
    ) -> Result<Option<HypSessionChange<'_, TBridge>>, HypSessionStateError<TBridge>>
    {
        self.process_event(event)
            .map_err(|error_event| HypSessionStateError::UnexpectedEvent { event: error_event })
    }

    fn process_event(&mut self, event: HypSessionEvent<TBridge>) -> ChangeResult<'_, TBridge>
    {
        log::info!("process_event: {}", event);

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
                    HypSessionEvent::RunError(run_error) => self.run_error(run_error),
                    _ => Err(event)
                }
            }
        }
    }

    fn start_run(&mut self) -> ChangeResult<'_, TBridge>
    {
        self.activity = HypState::Running;

        Ok(None)
    }

    fn complete_run(&mut self) -> ChangeResult<'_, TBridge>
    {
        self.activity = HypState::Passed;

        Ok(None)
    }

    fn output(&mut self, output: OutputReport<TBridge>) -> ChangeResult<'_, TBridge>
    {
        match self.hyps.entry(output.id().chain()).or_none()
        {
            Some(_) => todo!(),
            None => Err(HypSessionEvent::Output(output))
        }
    }

    fn hyp_exists(&mut self, hyp_node_info: TBridge::HypInfo) -> ChangeResult<'_, TBridge>
    {
        let hyp: Hyp<TBridge> = Hyp::new(hyp_node_info);

        self.hyps.insert(hyp);

        Ok(None)
    }

    fn run_error(
        &mut self,
        run_error: <TBridge as Bridge>::RunError
    ) -> Result<Option<HypSessionChange<'_, TBridge>>, HypSessionEvent<TBridge>>
    {
        self.activity = HypState::Failed;

        Ok(None)
    }
}
