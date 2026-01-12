use mockall::mock;
use passivate_delegation::Tx;

use crate::bridge::Bridge;
use crate::hyp_session_event::HypSessionEvent;
use crate::output_report::OutputReport;

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

pub trait RunErrorBridge<TBridge: Bridge>: Send + Sync + 'static
{
    fn run_error(&mut self, run_error: TBridge::RunError);
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

impl<TTx, TBridge: Bridge> RunErrorBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypSessionEvent<TBridge>> + Send + Sync + 'static
{
    fn run_error(&mut self, run_error: TBridge::RunError)
    {
        log::info!("run_error");
        self.send(HypSessionEvent::RunError(run_error));
    }
}

mock! {
    pub HypSessionBridge<TBridge: Bridge> { }

    impl<TBridge: Bridge> StartRunBridge<TBridge> for HypSessionBridge<TBridge>
    {
        fn start_run(&mut self);
    }

    impl<TBridge: Bridge> SendOutputBridge<TBridge> for HypSessionBridge<TBridge>
    {
        fn send_output(&mut self, output: OutputReport<TBridge>);
    }

    impl<TBridge: Bridge> SendHypBridge<TBridge> for HypSessionBridge<TBridge>
    {
        fn send_hyp(&mut self, hyp_info: TBridge::HypInfo);
    }

    impl<TBridge: Bridge> CompleteRunBridge<TBridge> for HypSessionBridge<TBridge>
    {
        fn complete_run(&mut self);
    }

    impl<TBridge: Bridge> RunErrorBridge<TBridge> for HypSessionBridge<TBridge>
    {
        fn run_error(&mut self, run_error: TBridge::RunError);
    }
}
