use std::thread;
use std::time::Duration;

use crossbeam_channel::select;
use passivate_delegation::Rx;
use passivate_hyp_names::hyp_id::HypId;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_request::{HypRunOptions, HypRunRequest, HypRunRequestKind};
use passivate_model_bridge::hyp_session_bridge::{
    CancelRunBridge,
    CompleteRunBridge,
    RunErrorBridge,
    SendHypBridge,
    SendOutputBridge,
    StartRunBridge
};
use tokio::sync::oneshot;

use crate::hyp_run_error::HypRunError;
use crate::hyp_runner::RunHyps;
use crate::model::RustBridge;

pub trait HypSessionBridge<TBridge: Bridge> = StartRunBridge<TBridge>
    + SendHypBridge<TBridge>
    + SendOutputBridge<TBridge>
    + CompleteRunBridge<TBridge>
    + CancelRunBridge<TBridge>
    + RunErrorBridge<TBridge>;

pub fn hyp_run_thread<'scope, 'env>(
    scope: &'scope thread::Scope<'scope, 'env>,
    mut hyp_run_trigger_rx: tokio::sync::mpsc::UnboundedReceiver<HypRunRequest<RustBridge>>,
    mut hyp_session_bridge: impl HypSessionBridge<RustBridge>,
    mut runner: impl RunHyps + Send + Sync + 'static
) -> thread::ScopedJoinHandle<'scope, ()>
{
    eprintln!("start hyp_run_thread");

    scope.spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .worker_threads(1)
            .enable_time()
            .build()
            .unwrap();

        runtime.block_on(async {
            loop
            {
                eprintln!("LOOP");

                tokio::select! {
                    request = hyp_run_trigger_rx.recv() => {
                        eprintln!("RECV: {request:?}");
                        //*current = countdown();
                        //let trigger = hyp_run_trigger_rx.borrow_and_update();

                        match request
                        {
                            Some(request) =>
                            {
                                hyp_session_bridge.start_run();
                                countdown(request).await;
                            },
                            None => {
                                hyp_session_bridge.complete_run();
                                eprintln!("BREAK");
                                break;
                            }
                        };
                        //*current = handle_hyp_run_trigger(&mut runner, &mut hyp_session_bridge, trigger);
                    }
                }
            }

            eprintln!("EXIT LOOP");
        });

        eprintln!("EXIT BLOCKON");
    })
}

async fn countdown(request: HypRunRequest<RustBridge>)
{
    eprintln!("start {request:?}");
    eprintln!("3");
    tokio::time::sleep(Duration::from_secs(1)).await;
    eprintln!("2");
    tokio::time::sleep(Duration::from_secs(1)).await;
    eprintln!("1");
    tokio::time::sleep(Duration::from_secs(1)).await;
    eprintln!("done {request:?}");
}

pub async fn handle_hyp_run_trigger(
    runner: &mut impl RunHyps,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    request: HypRunRequest<RustBridge>
)
{
    eprintln!("handle_hyp_run_trigger start");

    hyp_session_bridge.start_run();

    let result = match request.kind
    {
        HypRunRequestKind::All => run_hyps(runner, hyp_session_bridge, request.options.compute_coverage).await,
        HypRunRequestKind::Single { hyp_id } =>
        {
            run_hyp(runner, hyp_session_bridge, &hyp_id, request.options.update_snapshots).await
        }
    };

    match result
    {
        Ok(_) =>
        {
            eprintln!("complete_run");
            hyp_session_bridge.complete_run();
        }
        Err(test_error) =>
        {
            eprintln!("run_error");
            hyp_session_bridge.run_error(test_error);
        }
    };
}

async fn run_hyps(
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

async fn run_hyp(
    runner: &mut impl RunHyps,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    id: &HypId,
    update_snapshots: bool
) -> Result<(), HypRunError>
{
    runner.run_hyp(id, update_snapshots, hyp_session_bridge)
}
