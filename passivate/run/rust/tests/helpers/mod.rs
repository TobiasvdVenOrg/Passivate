use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_bridge::MockHypSessionBridge;
use passivate_run_rust::hyp_run_handler::{self, HypSessionBridge};
use passivate_run_rust::hyp_runner::{MockRunHyps, RunHyps};
use passivate_run_rust::model::RustBridge;
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

/// Convenience builder to invoke 'hyp_run_handler::handle_request'
pub struct HandleHypRunRequest<TRunHyps, THypSessionBridge>
{
    run_hyps: TRunHyps,
    hyp_session_bridge: THypSessionBridge,
    runtime: Option<Runtime>,
    cancellation: Option<CancellationToken>
}

impl HandleHypRunRequest<MockRunHyps, MockHypSessionBridge<RustBridge>>
{
    pub fn new() -> Self
    {
        let mock_hyp_session_bridge = MockHypSessionBridge::new();
        let mut mock_run_hyps = MockRunHyps::new();

        mock_run_hyps
            .expect_run_hyps::<MockHypSessionBridge<RustBridge>>()
            .returning(|_, _| Ok(()));

        mock_run_hyps
            .expect_run_hyp::<MockHypSessionBridge<RustBridge>>()
            .returning(|_, _, _| Ok(()));

        Self {
            run_hyps: mock_run_hyps,
            hyp_session_bridge: mock_hyp_session_bridge,
            runtime: None,
            cancellation: None
        }
    }
}

impl<_TRunHyps, _THypSessionBridge> HandleHypRunRequest<_TRunHyps, _THypSessionBridge>
{
    pub fn with_hyp_session_bridge<THypSessionBridge>(
        self,
        hyp_session_bridge: THypSessionBridge
    ) -> HandleHypRunRequest<_TRunHyps, THypSessionBridge>
    where
        THypSessionBridge: HypSessionBridge<RustBridge>
    {
        HandleHypRunRequest::<_TRunHyps, THypSessionBridge> {
            run_hyps: self.run_hyps,
            hyp_session_bridge,
            runtime: self.runtime,
            cancellation: self.cancellation
        }
    }

    pub fn with_runner<TRunHyps>(self, run_hyps: TRunHyps) -> HandleHypRunRequest<TRunHyps, _THypSessionBridge>
    where
        TRunHyps: RunHyps
    {
        HandleHypRunRequest::<TRunHyps, _THypSessionBridge> {
            run_hyps,
            hyp_session_bridge: self.hyp_session_bridge,
            runtime: self.runtime,
            cancellation: self.cancellation
        }
    }
}

impl<TRunHyps, THypSessionBridge> HandleHypRunRequest<TRunHyps, THypSessionBridge>
where
    TRunHyps: RunHyps + Send + Sync + 'static,
    THypSessionBridge: HypSessionBridge<RustBridge>
{
    pub fn call(&mut self, request: HypRunRequest<RustBridge>)
    {
        let runtime = self.runtime.get_or_insert(hyp_run_handler::build_tokio_runtime());
        let cancellation = self.cancellation.get_or_insert(CancellationToken::new()).child_token();

        runtime.block_on(async {
            hyp_run_handler::handle_request(&mut self.hyp_session_bridge, &mut self.run_hyps, request, cancellation).await;
        });
    }
}
