use std::thread;

use passivate_delegation::{CancellableMessage, Rx};
use passivate_hyp_names::hyp_id::HypId;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_request::{HypRunRequest, HypRunRequestKind};
use passivate_model_bridge::hyp_session_bridge::{
    CompleteRunBridge,
    RunErrorBridge,
    SendHypBridge,
    SendOutputBridge,
    StartRunBridge
};

use crate::hyp_run_error::HypRunError;
use crate::hyp_runner::RunHyps;
use crate::model::RustBridge;

pub trait HypSessionBridge<TBridge: Bridge> = StartRunBridge<TBridge>
    + SendHypBridge<TBridge>
    + SendOutputBridge<TBridge>
    + CompleteRunBridge<TBridge>
    + RunErrorBridge<TBridge>;

pub fn hyp_run_thread<'scope, 'env>(
    scope: &'scope thread::Scope<'scope, 'env>,
    hyp_run_trigger_rx: impl Rx<CancellableMessage<HypRunRequest<RustBridge>>> + 'env,
    mut hyp_session_bridge: impl HypSessionBridge<RustBridge>,
    mut runner: impl RunHyps + Send + Sync + 'static
)
{
    scope.spawn(move || {
        while let Ok(trigger) = hyp_run_trigger_rx.recv()
        {
            handle_hyp_run_trigger(&mut runner, &mut hyp_session_bridge, trigger.message);
        }
    });
}

pub fn handle_hyp_run_trigger(
    runner: &mut impl RunHyps,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    request: HypRunRequest<RustBridge>
)
{
    hyp_session_bridge.start_run();

    let result = match request.kind
    {
        HypRunRequestKind::All => run_hyps(runner, hyp_session_bridge, request.options.compute_coverage),
        HypRunRequestKind::Single { hyp_id } => run_hyp(runner, hyp_session_bridge, &hyp_id, request.options.update_snapshots)
    };

    match result
    {
        Ok(_) => hyp_session_bridge.complete_run(),
        Err(test_error) => hyp_session_bridge.run_error(test_error)
    };
}

fn run_hyps(
    runner: &mut impl RunHyps,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    compute_coverage: bool
) -> Result<(), HypRunError>
{
    // if compute_coverage
    // {
    //     self.coverage_tx.send(CoverageStatus::Preparing);
    // }

    // if let Err(clean_error) = self.coverage.clean_coverage_output()
    // {
    //     log::error!("error cleaning coverage output: {:?}", clean_error);
    // }

    runner.run_hyps(compute_coverage, hyp_session_bridge, Vec::new())
}

fn run_hyp(
    runner: &mut impl RunHyps,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    id: &HypId,
    update_snapshots: bool
) -> Result<(), HypRunError>
{
    runner.run_hyp(id, update_snapshots, hyp_session_bridge)
}
