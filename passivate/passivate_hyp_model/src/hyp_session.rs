use crate::hyp_run_events::HypRunChange;
use crate::hyp_session_event::HypSessionEvent;
use crate::hyp_session_state::{HypSessionState, HypSessionStateError};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Activity
{
    Idle,
    Running
}

#[derive(Debug, Clone)]
pub struct HypSession
{
    activity: Result<Activity, HypSessionStateError>
}

impl HypSession
{
    pub fn new() -> Self
    {
        Self {
            activity: Ok(Activity::Idle)
        }
    }

    pub fn from_events(events: impl IntoIterator<Item = HypSessionEvent>) -> Self
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

    fn evaluate_state(activity: &Activity) -> HypSessionState
    {
        match activity
        {
            Activity::Idle => HypSessionState::Idle,
            Activity::Running => HypSessionState::Running
        }
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

        self.activity = match (&self.activity, event)
        {
            (Err(error), _) => Err(error.clone()),
            (Ok(activity), event) => Self::process_event(activity, event)
        };

        change
    }

    fn process_event(activity: &Activity, event: HypSessionEvent) -> Result<Activity, HypSessionStateError>
    {
        match event
        {
            HypSessionEvent::RunStarted => Self::start_run(activity),
            HypSessionEvent::WorkspaceCompilation(workspace_compilation_event) => todo!(),
            HypSessionEvent::CrateExists => todo!(),
            HypSessionEvent::CrateCompilation(crate_compilation_event) => todo!(),
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
