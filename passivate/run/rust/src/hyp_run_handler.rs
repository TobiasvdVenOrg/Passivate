use std::pin::Pin;
use std::future;

use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_request::{HypRunRequest, HypRunRequestKind};
use passivate_model_bridge::hyp_session_bridge::{
    CancelRunBridge,
    CompleteRunBridge,
    RunErrorBridge,
    SendHypBridge,
    SendOutputBridge,
    StartRunBridge
};
use tokio_util::sync::CancellationToken;

use crate::hyp_run_error::HypRunError;
use crate::hyp_runner::RunHyps;
use crate::model::RustBridge;

pub trait HypSessionBridge<TBridge: Bridge> = StartRunBridge<TBridge>
    + SendHypBridge<TBridge>
    + SendOutputBridge<TBridge>
    + CompleteRunBridge<TBridge>
    + CancelRunBridge<TBridge>
    + RunErrorBridge<TBridge>;

async fn pending_hyp_run<THypSessionBridge, TRunHyps>(hyp_session_bridge: THypSessionBridge, run_hyps: TRunHyps, cancellation: CancellationToken) -> (THypSessionBridge, TRunHyps)
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps + Send
{
    cancellation.run_until_cancelled(future::pending::<!>()).await;

    (hyp_session_bridge, run_hyps)
}

pub async fn handle_request<THypSessionBridge, TRunHyps>(
    hyp_session_bridge: &mut THypSessionBridge,
    run_hyps: &mut TRunHyps,
    request: HypRunRequest<RustBridge>,
    cancellation: CancellationToken
)
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps + Send + Sync + 'static
{
    hyp_session_bridge.start_run();

    let task: Pin<Box<dyn Future<Output = Result<(), HypRunError>> + Send>> = match request.kind
    {
        HypRunRequestKind::All => Box::pin(run_hyps.run_hyps(request.options.compute_coverage, hyp_session_bridge, Vec::new())),
        HypRunRequestKind::Single { hyp_id } =>
        {
            Box::pin(run_hyps.run_hyp( hyp_id, request.options.update_snapshots,  hyp_session_bridge))
        }
    };

    let result = cancellation.run_until_cancelled(task).await;

    match result
    {
        Some(Ok(_)) =>
        {
            hyp_session_bridge.complete_run();
        }
        Some(Err(test_error)) =>
        {
            hyp_session_bridge.run_error(test_error);
        }
        None =>
        {
            hyp_session_bridge.cancel_run();
        }
    };
}

async fn handle_request_take<THypSessionBridge, TRunHyps>(
    mut hyp_session_bridge: THypSessionBridge,
    mut run_hyps: TRunHyps,
    request: HypRunRequest<RustBridge>,
    cancellation: CancellationToken
) -> (THypSessionBridge, TRunHyps)
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps + Send + Sync + 'static
{
    handle_request(&mut hyp_session_bridge, &mut run_hyps, request, cancellation).await;

    (hyp_session_bridge, run_hyps)
}

pub fn build_tokio_runtime() -> tokio::runtime::Runtime
{
    tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        .enable_time()
        .build()
        .unwrap()
}

pub fn spawn_hyp_run_future<THypSessionBridge, TRunHyps>(
    runtime: &tokio::runtime::Runtime,
    mut hyp_run_trigger_rx: tokio::sync::mpsc::UnboundedReceiver<HypRunRequest<RustBridge>>,
    hyp_session_bridge: THypSessionBridge,
    run_hyps: TRunHyps
)
-> tokio::task::JoinHandle<()>
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps + Send + Sync + 'static
{
    runtime.spawn(async move {
        let mut cancellation = tokio_util::sync::CancellationToken::new();

        let mut running_request: Pin<Box<dyn Future<Output = (THypSessionBridge, TRunHyps)> + Send>> =
            Box::pin(pending_hyp_run(hyp_session_bridge, run_hyps, cancellation.child_token()));

        loop
        {
            tokio::select! {
                request = hyp_run_trigger_rx.recv() => {

                    match request
                    {
                        Some(request) =>
                        {
                            // New request, cancel a running request first and retrieve the context, then start the handling the new request
                            cancellation.cancel();
                            let (hyp_session_bridge, run_hyps) = running_request.await;
                            cancellation = CancellationToken::new();
                            running_request = Box::pin(handle_request_take(hyp_session_bridge, run_hyps, request, cancellation.child_token()));
                        },
                        None => {
                            // Channel closed, cancel a running request
                            cancellation.cancel();
                            _ = running_request.await;
                            break;
                        }
                    };
                }

                (hyp_session_bridge, run_hyps) = running_request.as_mut() => {
                    // A request completed
                    running_request = Box::pin(pending_hyp_run(hyp_session_bridge, run_hyps, cancellation.child_token()));
                }
            };
        };
    })
}

pub async fn handle_hyp_run_trigger(
    runner: &mut impl RunHyps,
    hyp_session_bridge: &mut impl HypSessionBridge<RustBridge>,
    request: HypRunRequest<RustBridge>
)
{
    eprintln!("handle_hyp_run_trigger start");
}

pub async fn handle_hyp_run_trigger2<THypSessionBridge, TRunHyps>(
    runner: TRunHyps,
    hyp_session_bridge: THypSessionBridge,
    request: HypRunRequest<RustBridge>,
    cancellation: CancellationToken
)
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps + Send + Sync + 'static
{
    eprintln!("handle_hyp_run_trigger start");
}
