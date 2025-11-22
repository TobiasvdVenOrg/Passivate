use std::fmt::Debug;
use std::marker::PhantomData;
use std::{iter, slice};

use crate::bridge::{Bridge, ProjectId};
use crate::hyp::Hyp;
use crate::hyp_iter_ext::HypIterator;
use crate::hyp_session_change::HypSessionChange;
use crate::hyp_session_event::HypSessionEvent;
use crate::hyp_session_state_error::HypSessionStateError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypSessionActivity
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
pub struct Project<TBridge: Bridge>
{
    pub info: TBridge::TProjectInfo,
    pub compilation: Vec<TBridge::TProjectCompilation>
}

#[derive(Debug, Clone)]
struct Session<TBridge: Bridge>
{
    activity: HypSessionActivity,
    projects: Vec<Project<TBridge>>,
    workspace_compilation: Vec<TBridge::TWorkspaceCompilation>,
    bridge: PhantomData<TBridge>
}

type ChangeResult<'a, TBridge> = Result<Option<HypSessionChange<'a, TBridge>>, HypSessionEvent<TBridge>>;

impl<TBridge: Bridge> HypSession<TBridge>
{
    pub fn new() -> Self
    {
        let session = Session {
            activity: HypSessionActivity::Idle,
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

    pub fn activity(&self) -> Result<&HypSessionActivity, &HypSessionStateError<TBridge>>
    {
        self.error.as_ref().map_or_else(|| Ok(&self.session.activity), |e| Err(e))
    }

    pub fn projects(&self) -> impl Iterator<Item = &Project<TBridge>>
    {
        match self.error
        {
            Some(_) => slice::Iter::default(),
            None => self.session.projects.iter()
        }
    }

    pub fn project_infos(&self) -> impl Iterator<Item = &TBridge::TProjectInfo>
    {
        self.projects().map(|p| &p.info)
    }

    pub fn all_hyps(&self) -> impl HypIterator<'_> + Debug
    {
        iter::empty::<&&Hyp>()
    }

    pub fn last_workspace_compilation(&self) -> Option<&TBridge::TWorkspaceCompilation>
    {
        match self.error
        {
            Some(_) => None,
            None => self.session.workspace_compilation.last()
        }
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
        match self.activity
        {
            HypSessionActivity::Idle =>
            {
                match event
                {
                    HypSessionEvent::RunStarted => self.start_run(),
                    _ => Err(event)
                }
            }
            HypSessionActivity::Running =>
            {
                match event
                {
                    HypSessionEvent::ProjectExists(project) => self.project_exists(project),
                    HypSessionEvent::WorkspaceCompilation(workspace_compilation) =>
                    {
                        self.workspace_compilation(workspace_compilation)
                    }
                    HypSessionEvent::ProjectCompilation(project_compilation) => self.project_compilation(project_compilation),
                    HypSessionEvent::RunCompleted => self.complete_run(),
                    _ => Err(event)
                }
            }
        }
    }

    fn start_run(&mut self) -> ChangeResult<'_, TBridge>
    {
        self.activity = HypSessionActivity::Running;

        Ok(None)
    }

    fn complete_run(&mut self) -> ChangeResult<'_, TBridge>
    {
        self.activity = HypSessionActivity::Idle;

        Ok(None)
    }

    fn project_exists(&mut self, project_info: TBridge::TProjectInfo) -> ChangeResult<'_, TBridge>
    {
        let added = self.projects.push_mut(Project {
            info: project_info,
            compilation: Vec::new()
        });

        Ok(Some(HypSessionChange::NewProject(added)))
    }

    fn workspace_compilation(&mut self, workspace_compilation: TBridge::TWorkspaceCompilation) -> ChangeResult<'_, TBridge>
    {
        self.workspace_compilation.push(workspace_compilation);

        Ok(None)
    }

    fn project_compilation(&mut self, project_compilation: TBridge::TProjectCompilation) -> ChangeResult<'_, TBridge>
    {
        let project = self.projects.iter_mut().find(|p| p.info.id() == project_compilation.id());

        match project
        {
            Some(p) =>
            {
                p.compilation.push(project_compilation);
                Ok(None)
            }
            None => Err(HypSessionEvent::ProjectCompilation(project_compilation))
        }
    }
}
