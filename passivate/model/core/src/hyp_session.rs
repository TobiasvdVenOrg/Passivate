use std::collections::HashSet;
use std::fmt::Debug;
use std::iter::{self, Empty};
use std::marker::PhantomData;

use crate::bridge::Bridge;
use crate::hyp::Hyp;
use crate::hyp_iter_ext::HypIterator;
use crate::hyp_session_change::HypSessionChange;
use crate::hyp_session_event::HypSessionEvent;
use crate::hyp_session_state::{HypSessionState, HypSessionStateError};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Activity
{
    Idle,
    Running
}

#[derive(Debug, Clone)]
pub struct HypSession<TBridge: Bridge>
{
    activity: Result<Activity, HypSessionStateError>,
    bridge: PhantomData<TBridge>
}

impl<TBridge: Bridge> HypSession<TBridge>
{
    pub fn new() -> Self
    {
        Self {
            activity: Ok(Activity::Idle),
            bridge: PhantomData
        }
    }

    pub fn from_events(events: impl IntoIterator<Item = HypSessionEvent<TBridge>>) -> Self
    {
        let mut session = Self::new();
        session.update_all(events);
        session
    }

    pub fn state(&self) -> Result<HypSessionState, &HypSessionStateError>
    {
        match &self.activity
        {
            Ok(activity) => Ok(Self::evaluate_state(activity)),
            Err(error) => Err(error)
        }
    }

    pub fn projects(&self) -> Vec<TBridge::TProject>
    {
        Vec::new()
    }

    fn evaluate_state(activity: &Activity) -> HypSessionState
    {
        match activity
        {
            Activity::Idle => HypSessionState::Idle,
            Activity::Running => HypSessionState::Running
        }
    }

    pub fn all_hyps(&self) -> impl HypIterator<'_> + Debug
    {
        iter::empty::<&&Hyp>()
    }

    pub fn update_all(&mut self, events: impl IntoIterator<Item = HypSessionEvent<TBridge>>)
    {
        for event in events
        {
            self.update(event);
        }
    }

    pub fn update(&mut self, event: HypSessionEvent<TBridge>) -> Option<HypSessionChange<'_>>
    {
        let mut change = None;

        self.activity = match (&self.activity, event)
        {
            (Err(_), _) => return None,
            (Ok(activity), event) => Self::process_event(activity, event)
        };

        change
    }

    fn process_event(activity: &Activity, event: HypSessionEvent<TBridge>) -> Result<Activity, HypSessionStateError>
    {
        match event
        {
            HypSessionEvent::RunStarted => Self::start_run(activity),
            HypSessionEvent::ProjectExists(hyp_crate) => todo!(),
            HypSessionEvent::WorkspaceCompilation(workspace_compilation_event) => todo!(),
            HypSessionEvent::ProjectCompilation(crate_compilation_event) => todo!(),
            HypSessionEvent::HypExists(hyp_id) => todo!(),
            HypSessionEvent::HypRunning(hyp_id) => todo!(),
            HypSessionEvent::HypStdOut { id, lines } => todo!(),
            HypSessionEvent::HypStdErr { id, lines } => todo!(),
            HypSessionEvent::HypCompleted(hyp_id) => todo!(),
            HypSessionEvent::RunCompleted => Self::complete_run(activity)
        }
    }

    fn start_run(current_activity: &Activity) -> Result<Activity, HypSessionStateError>
    {
        if *current_activity == Activity::Idle
        {
            Ok(Activity::Running)
        }
        else
        {
            Err(HypSessionStateError::UnexpectedStart)
        }
    }

    fn complete_run(current_activity: &Activity) -> Result<Activity, HypSessionStateError>
    {
        if *current_activity == Activity::Running
        {
            Ok(Activity::Idle)
        }
        else
        {
            Err(HypSessionStateError::UnexpectedCompletion)
        }
    }
}
