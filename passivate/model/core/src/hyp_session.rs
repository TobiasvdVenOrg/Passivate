use std::fmt::Debug;
use std::marker::PhantomData;
use std::{iter, slice};

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
    session: Session<TBridge>,
    error: Option<HypSessionStateError<TBridge>>
}

#[derive(Debug, Clone)]
struct Session<TBridge: Bridge>
{
    activity: Activity,
    projects: Vec<TBridge::TProject>,
    workspace_compilation: Vec<TBridge::TWorkspaceCompilation>,
    bridge: PhantomData<TBridge>
}

impl<TBridge: Bridge> HypSession<TBridge>
{
    pub fn new() -> Self
    {
        let session = Session {
            activity: Activity::Idle,
            projects: Vec::new(),
            workspace_compilation: Vec::new(),
            bridge: PhantomData
        };

        HypSession { session, error: None }
    }

    pub fn from_events(events: impl IntoIterator<Item = HypSessionEvent<TBridge>>) -> Self
    {
        let mut session = Self::new();
        session.update_all(events);
        session
    }

    pub fn state(&self) -> Result<HypSessionState, &HypSessionStateError<TBridge>>
    {
        self.error
            .as_ref()
            .map_or_else(|| Ok(self.session.evaluate_state()), |e| Err(e))
    }

    pub fn projects(&self) -> impl Iterator<Item = &TBridge::TProject>
    {
        match self.error
        {
            Some(_) => slice::Iter::default(),
            None => self.session.projects.iter()
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
}

impl<TBridge: Bridge> Session<TBridge>
{
    fn evaluate_state(&self) -> HypSessionState
    {
        match self.activity
        {
            Activity::Idle => HypSessionState::Idle,
            Activity::Running =>
            {
                if self.workspace_compilation.is_empty()
                {
                    HypSessionState::Starting
                }
                else
                {
                    HypSessionState::Compiling
                }
            }
        }
    }

    fn update(
        &mut self,
        event: HypSessionEvent<TBridge>
    ) -> Result<Option<HypSessionChange<'_, TBridge>>, HypSessionStateError<TBridge>>
    {
        let current_state = self.evaluate_state();

        self.process_event(&current_state, event).map_err(|error_event| {
            HypSessionStateError::UnexpectedEvent {
                state: current_state,
                event: error_event
            }
        })
    }

    fn process_event(
        &mut self,
        current_state: &HypSessionState,
        event: HypSessionEvent<TBridge>
    ) -> Result<Option<HypSessionChange<'_, TBridge>>, HypSessionEvent<TBridge>>
    {
        match self.activity
        {
            Activity::Idle =>
            {
                match event
                {
                    HypSessionEvent::RunStarted =>
                    {
                        self.start_run();
                        Ok(None)
                    }
                    _ => Err(event)
                }
            }
            Activity::Running =>
            {
                match event
                {
                    HypSessionEvent::ProjectExists(project) => Ok(Some(self.project_exists(project))),
                    HypSessionEvent::WorkspaceCompilation(workspace_compilation) =>
                    {
                        self.workspace_compilation(workspace_compilation);
                        Ok(None)
                    }
                    HypSessionEvent::RunCompleted =>
                    {
                        self.complete_run();
                        Ok(None)
                    }
                    _ => Err(event)
                }
            }
        }
    }

    fn start_run<'a, 'c>(&mut self)
    {
        self.activity = Activity::Running;
    }

    fn complete_run(&mut self)
    {
        self.activity = Activity::Idle;
    }

    fn project_exists(&mut self, project: TBridge::TProject) -> HypSessionChange<'_, TBridge>
    {
        let added = self.projects.push_mut(project);

        HypSessionChange::NewProject(added)
    }

    fn workspace_compilation(&mut self, workspace_compilation: TBridge::TWorkspaceCompilation)
    {
        self.workspace_compilation.push(workspace_compilation);
    }
}
