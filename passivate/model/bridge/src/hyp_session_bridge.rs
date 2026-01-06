use mockall::mock;
use passivate_delegation::Tx;

use crate::bridge::Bridge;
use crate::hyp_session_event::HypSessionEvent;
use crate::output_report::OutputReport;

/// Interface from non-core systems (notify, configuration) to communicate to the session that something has changed
/// and tests may need to be re-discovered and/or run
pub trait HypSessionBridge
{
    fn request_rerun(&self);
}

pub enum HypSessionTrigger
{
    RequestRerun
}

impl<TTx> HypSessionBridge for TTx
where
    TTx: Tx<HypSessionTrigger>
{
    fn request_rerun(&self)
    {
        self.send(HypSessionTrigger::RequestRerun);
    }
}

/// Interfaces from a test runner implementation to communicate changes to the session state.
pub trait StartRunBridge<TBridge: Bridge>: Send + Sync + 'static
{
    fn start_run(&mut self);
}

pub trait SendOutputBridge<TBridge: Bridge>: Send + Sync + 'static
{
    fn send_output(&mut self, output: OutputReport<TBridge>);
}

pub trait SendHypBridge<TBridge: Bridge>: Send + Sync + 'static
{
    fn send_hyp(&mut self, hyp_info: TBridge::HypInfo);
}

pub trait CompleteRunBridge<TBridge: Bridge>: Send + Sync + 'static
{
    fn complete_run(&mut self);
}

impl<TTx, TBridge> StartRunBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypSessionEvent<TBridge>> + Send + Sync + 'static
{
    fn start_run(&mut self)
    {
        log::info!("start_run");
        self.send(HypSessionEvent::RunStarted);
    }
}

impl<TTx, TBridge: Bridge> SendOutputBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypSessionEvent<TBridge>> + Send + Sync + 'static
{
    fn send_output(&mut self, report: OutputReport<TBridge>)
    {
        log::info!("send_output");
        self.send(HypSessionEvent::Output(report));
    }
}

impl<TTx, TBridge: Bridge> SendHypBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypSessionEvent<TBridge>> + Send + Sync + 'static
{
    fn send_hyp(&mut self, hyp_info: TBridge::HypInfo)
    {
        log::info!("send_hyp");
        self.send(HypSessionEvent::HypExists(hyp_info));
    }
}

impl<TTx, TBridge: Bridge> CompleteRunBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypSessionEvent<TBridge>> + Send + Sync + 'static
{
    fn complete_run(&mut self)
    {
        log::info!("complete_run");
        self.send(HypSessionEvent::RunCompleted);
    }
}

mock! {
    MockHypSessionBridge<TBridge: Bridge> { }

    impl<TBridge: Bridge> StartRunBridge<TBridge> for MockHypSessionBridge<TBridge>
    {
        fn start_run(&mut self);
    }

    impl<TBridge: Bridge> SendOutputBridge<TBridge> for MockHypSessionBridge<TBridge>
    {
        fn send_output(&mut self, output: OutputReport<TBridge>);
    }

    impl<TBridge: Bridge> SendHypBridge<TBridge> for MockHypSessionBridge<TBridge>
    {
        fn send_hyp(&mut self, hyp_info: TBridge::HypInfo);
    }

    impl<TBridge: Bridge> CompleteRunBridge<TBridge> for MockHypSessionBridge<TBridge>
    {
        fn complete_run(&mut self);
    }
}
