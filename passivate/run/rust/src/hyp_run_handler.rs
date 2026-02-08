use std::pin::Pin;
use std::time::Duration;
use std::future;

use passivate_hyp_names::hyp_id::HypId;
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

struct HypRunContext<THypSessionBridge, TRunHyps>
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps
{
    hyp_session_bridge: THypSessionBridge,
    run_hyps: TRunHyps
}

impl<THypSessionBridge, TRunHyps> HypRunContext<THypSessionBridge, TRunHyps>
where
    THypSessionBridge: HypSessionBridge<RustBridge>,
    TRunHyps: RunHyps
{
    async fn pending_hyp_run(self, cancellation: CancellationToken) -> Self
    {
        let pending = future::pending();

        tokio::select! {
            _ = pending => {
                unreachable!()
            }

            _ = cancellation.cancelled() => {

            }
        };

        self
    }

    async fn countdown(mut self, request: HypRunRequest<RustBridge>, cancellation: CancellationToken) -> Self
    {
        eprintln!("start {request:?}");
        self.hyp_session_bridge.start_run();

        for count in (0 .. 3).rev().map(|c| c + 1)
        {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    eprintln!("{count}");
                }

                _ = cancellation.cancelled() => {
                    eprintln!("CANCEL");
                    self.hyp_session_bridge.cancel_run();
                    return self;
                }
            };
        }

        eprintln!("done {request:?}");

        self.hyp_session_bridge.complete_run();

        self
    }
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
        let context = HypRunContext {
            hyp_session_bridge,
            run_hyps
        };
        let mut running_request: Pin<Box<dyn Future<Output = HypRunContext<THypSessionBridge, TRunHyps>> + Send>> =
            Box::pin(context.pending_hyp_run(cancellation.child_token()));

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
                            let context = running_request.await;
                            cancellation = CancellationToken::new();
                            running_request = Box::pin(context.countdown(request, cancellation.child_token()));
                        },
                        None => {
                            // Channel closed, cancel a running request
                            cancellation.cancel();
                            _ = running_request.await;
                            break;
                        }
                    };
                }

                context = running_request.as_mut() => {
                    // A request completed
                    running_request = Box::pin(context.pending_hyp_run(cancellation.child_token()));
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
