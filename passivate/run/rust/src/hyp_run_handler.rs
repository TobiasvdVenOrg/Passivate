use std::thread;

use passivate_delegation::{CancellableMessage, Cancellation, Rx};
use passivate_hyp_names::hyp_id::HypId;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_request::{HypRunRequest, HypRunRequestKind};
use passivate_model_bridge::hyp_session_bridge::{CompleteRunBridge, SendHypBridge, SendOutputBridge, StartRunBridge};
use passivate_model_rust::RustBridge;

use crate::hyp_runner::HypRunner;

pub trait HypSessionBridge<TBridge: Bridge> =
    StartRunBridge<TBridge> + SendHypBridge<TBridge> + SendOutputBridge<TBridge> + CompleteRunBridge<TBridge>;

pub fn hyp_run_thread<'scope, 'env>(
    scope: &'scope thread::Scope<'scope, 'env>,
    hyp_run_trigger_rx: impl Rx<CancellableMessage<HypRunRequest<RustBridge>>> + 'env,
    mut hyp_session_bridge: impl HypSessionBridge<RustBridge>,
    mut runner: HypRunner
)
{
    scope.spawn(move || {
        while let Ok(trigger) = hyp_run_trigger_rx.recv()
        {
            hyp_session_bridge.start_run();
            handle_hyp_run_trigger(&mut runner, &mut hyp_session_bridge, trigger.message, trigger.cancellation);
            hyp_session_bridge.complete_run();
        }
    });
}

pub fn handle_hyp_run_trigger(
    runner: &mut HypRunner,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    request: HypRunRequest<RustBridge>,
    cancellation: Cancellation
)
{
    match request.kind
    {
        HypRunRequestKind::All =>
        {
            run_hyps(
                runner,
                hyp_session_bridge,
                cancellation.clone(),
                request.options.compute_coverage
            )
        }
        HypRunRequestKind::Single { hyp_id } =>
        {
            run_hyp(
                runner,
                hyp_session_bridge,
                &hyp_id,
                request.options.update_snapshots,
                cancellation.clone()
            )
        }
    }
}

fn run_hyps(
    runner: &mut HypRunner,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    cancellation: Cancellation,
    compute_coverage: bool
)
{
    // if compute_coverage
    // {
    //     self.coverage_tx.send(CoverageStatus::Preparing);
    // }

    // if let Err(clean_error) = self.coverage.clean_coverage_output()
    // {
    //     log::error!("error cleaning coverage output: {:?}", clean_error);
    // }

    if cancellation.is_cancelled()
    {
        return;
    }

    let test_output = runner.run_hyps(compute_coverage, cancellation.clone(), hyp_session_bridge, Vec::new());

    if cancellation.is_cancelled()
    {
        return;
    }

    match test_output
    {
        Ok(_) =>
        {}
        Err(test_error) =>
        {
            todo!()
        }
    };
}

fn run_hyp(
    runner: &mut HypRunner,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    id: &HypId,
    update_snapshots: bool,
    cancellation: Cancellation
)
{
    let result = runner.run_hyp(id, update_snapshots, cancellation, hyp_session_bridge);

    if let Err(error) = result
    {
        todo!()
    }
}
