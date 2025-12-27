use passivate_delegation::{Rx, Tx};
use passivate_model_bridge::{Bridge, HypSessionBridge, OutputReport};
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
    fn start_run(&mut self)
    {
        self.tx.send(HypSessionEvent::RunStarted);
    }

    fn output(&mut self, report: OutputReport<TBridge>)
    {
        self.tx.send(HypSessionEvent::Output(report));
    }

    fn hyp(&mut self, hyp_info: TBridge::HypInfo)
    {
        self.tx.send(HypSessionEvent::HypExists(hyp_info));
    }

    fn complete_run(&mut self)
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
        stub._when_output().then(|_| {});
        stub._when_hyp().then(|_| {});
        stub._when_complete_run().then(|_| {});

        stub
    }
}
