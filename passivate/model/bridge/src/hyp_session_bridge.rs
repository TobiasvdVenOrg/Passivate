use crate::bridge::Bridge;
use crate::output_report::OutputReport;

/// Interface from a test runner implementation to communicate changes to the session state.
pub trait HypSessionBridge<TBridge: Bridge>
{
    fn start_run(&mut self);
    fn send_output(&mut self, output: OutputReport<TBridge>);
    fn send_hyp(&mut self, hyp_info: TBridge::HypInfo);
    fn complete_run(&mut self);
}
