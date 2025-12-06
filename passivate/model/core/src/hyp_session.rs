use std::fmt::Debug;

use indextree::Arena;

use crate::bridge::{Bridge, HypPath};
use crate::hyp::HypNode;
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
struct Session<TBridge: Bridge>
{
    activity: HypSessionActivity,
    hyp_nodes: Arena<TBridge::TId, HypNode<TBridge>>
}

type ChangeResult<TBridge> = Result<Option<HypSessionChange<TBridge>>, HypSessionEvent<TBridge>>;

impl<TBridge: Bridge> HypSession<TBridge>
{
    pub fn new() -> Self
    {
        let session = Session {
            activity: HypSessionActivity::Idle,
            hyp_nodes: Trie::new()
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
                    HypSessionEvent::Output(output) => self.output(output),
                    HypSessionEvent::HypNodeExists(project) => self.hyp_node_exists(project),
                    HypSessionEvent::RunCompleted => self.complete_run(),
                    _ => Err(event)
                }
            }
        }
    }

    fn start_run(&mut self) -> ChangeResult<TBridge>
    {
        self.activity = HypSessionActivity::Running;

        Ok(None)
    }

    fn complete_run(&mut self) -> ChangeResult<TBridge>
    {
        self.activity = HypSessionActivity::Idle;

        Ok(None)
    }

    fn output(&mut self, output: TBridge::TOutput) -> ChangeResult<TBridge>
    {
        todo!()
    }

    fn hyp_node_exists(&mut self, hyp_node_info: TBridge::THypNodeInfo) -> ChangeResult<TBridge>
    {
        let hyp_node: HypNode<TBridge> = HypNode { info: hyp_node_info };
        let key = hyp_node.path().clone();

        self.hyp_nodes.insert(key, hyp_node);

        Ok(None)
    }
}
