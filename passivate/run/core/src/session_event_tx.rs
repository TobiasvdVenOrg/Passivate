use passivate_delegation::{Rx, Tx};
use passivate_model_core::bridge::{Bridge, HypSessionBridge};
use passivate_model_core::hyp_session_event::HypSessionEvent;

#[faux::create]
pub struct SessionEventTx<TBridge: Bridge>
{
    tx: Tx<HypSessionEvent<TBridge>>
}

#[faux::methods]
impl<TBridge: Bridge> SessionEventTx<TBridge>
{
    pub fn new() -> (Self, Rx<HypSessionEvent<TBridge>>)
    {
        let (tx, rx) = Tx::new();

        (Self { tx }, rx)
    }
}

#[faux::methods]
impl<TBridge: Bridge> HypSessionBridge<TBridge> for SessionEventTx<TBridge>
{
    fn start_run(&self)
    {
        self.tx.send(HypSessionEvent::RunStarted);
    }

    fn project_exists(&self, project_info: TBridge::TProjectInfo)
    {
        self.tx.send(HypSessionEvent::ProjectExists(project_info));
    }

    fn complete_run(&self)
    {
        self.tx.send(HypSessionEvent::RunCompleted);
    }
}

impl<TBridge: Bridge> SessionEventTx<TBridge>
{
    pub fn stub() -> Self
    {
        let mut stub = SessionEventTx::faux();
        stub._when_start_run().then(|_| {});
        stub._when_project_exists().then(|_| {});
        stub._when_complete_run().then(|_| {});

        stub
    }
}
