use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_bridge::MockHypSessionBridge;
use passivate_run_rust::hyp_run_handler::{self, HypSessionBridge, build_tokio_runtime, handle_hyp_run_trigger};
use passivate_run_rust::hyp_runner::{HypRunner, MockRunHyps, RunHyps};
use passivate_run_rust::model::RustBridge;
use passivate_testing::test_data_setup::TestDataSetup;
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

/// Convenience builder to invoke 'handle_hyp_run_trigger'
pub struct HandleHypRunTrigger<TRunHyps, THypSessionBridge>
{
    run_hyps: TRunHyps,
    hyp_session_bridge: THypSessionBridge,
    runtime: Option<Runtime>,
    cancellation: Option<CancellationToken>
}

impl HandleHypRunTrigger<MockRunHyps, MockHypSessionBridge<RustBridge>>
{
    pub fn new() -> Self
    {
        let mock_hyp_session_bridge = MockHypSessionBridge::new();
        let mut mock_run_hyps = MockRunHyps::new();

        mock_run_hyps
            .expect_run_hyps::<MockHypSessionBridge<RustBridge>>()
            .returning(|_, _, _| Ok(()));

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

impl<_TRunHyps, _THypSessionBridge> HandleHypRunTrigger<_TRunHyps, _THypSessionBridge>
{
    pub fn with_hyp_session_bridge<THypSessionBridge>(
        self,
        hyp_session_bridge: THypSessionBridge
    ) -> HandleHypRunTrigger<_TRunHyps, THypSessionBridge>
    where
        THypSessionBridge: HypSessionBridge<RustBridge>
    {
        HandleHypRunTrigger::<_TRunHyps, THypSessionBridge> {
            run_hyps: self.run_hyps,
            hyp_session_bridge,
            runtime: self.runtime,
            cancellation: self.cancellation
        }
    }

    pub fn with_runner<TRunHyps>(self, run_hyps: TRunHyps) -> HandleHypRunTrigger<TRunHyps, _THypSessionBridge>
    where
        TRunHyps: RunHyps
    {
        HandleHypRunTrigger::<TRunHyps, _THypSessionBridge> {
            run_hyps,
            hyp_session_bridge: self.hyp_session_bridge,
            runtime: self.runtime,
            cancellation: self.cancellation
        }
    }

    pub fn with_runner_from_setup(self, setup: &TestDataSetup) -> HandleHypRunTrigger<HypRunner, _THypSessionBridge>
    {
        let runner = HypRunner::new(
            setup.workspace_path().clone(),
            setup.output_path().clone(),
            setup.coverage_path().clone()
        );

        HandleHypRunTrigger::<HypRunner, _THypSessionBridge> {
            run_hyps: runner,
            hyp_session_bridge: self.hyp_session_bridge,
            runtime: self.runtime,
            cancellation: self.cancellation
        }
    }
}

impl<TRunHyps, THypSessionBridge> HandleHypRunTrigger<TRunHyps, THypSessionBridge>
where
    TRunHyps: RunHyps + Send + Sync + 'static,
    THypSessionBridge: HypSessionBridge<RustBridge>
{
    pub fn call(&mut self, request: HypRunRequest<RustBridge>)
    {
        let runtime = self.runtime.get_or_insert(build_tokio_runtime());
        let cancellation = self.cancellation.get_or_insert(CancellationToken::new()).child_token();

        runtime.block_on(async {
            hyp_run_handler::handle_request(&mut self.hyp_session_bridge, &mut self.run_hyps, request, cancellation).await;
        });
    }
}
