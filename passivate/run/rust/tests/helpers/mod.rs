use std::env;

use passivate_coverage::grcov::Grcov;
use passivate_delegation::{Cancellation, MockTx, cancellation};
use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_bridge::{self, MockHypSessionBridge};
use passivate_model_rust::RustBridge;
use passivate_run_rust::hyp_run_handler::{HypSessionBridge, handle_hyp_run_trigger};
use passivate_run_rust::hyp_runner::{HypRunner, MockRunHyps, RunHyps};
use passivate_testing::test_data_setup::TestDataSetup;

#[bon::builder]
pub fn test_grcov(#[builder(start_fn)] setup: &TestDataSetup) -> Grcov
{
    Grcov::builder()
        .workspace_path(setup.workspace_path())
        .output_path(setup.coverage_path())
        .binary_path(setup.binary_path())
        .build()
}

#[bon::builder]
pub fn test_hyp_runner(#[builder(start_fn)] setup: &TestDataSetup) -> HypRunner
{
    let target = env::var("HOST").expect("expected 'HOST' environment target triple");

    HypRunner::new(
        target,
        setup.workspace_path().clone(),
        setup.output_path().clone(),
        setup.coverage_path().clone()
    )
}

/// Convenience builder to invoke 'handle_hyp_run_trigger'
pub struct HandleHypRunTrigger<TRunHyps, THypSessionBridge>
{
    run_hyps: TRunHyps,
    hyp_session_bridge: THypSessionBridge,
    cancellation: Cancellation
}

impl HandleHypRunTrigger<MockRunHyps, MockHypSessionBridge<RustBridge>>
{
    pub fn new() -> Self
    {
        let mock_hyp_session_bridge = MockHypSessionBridge::new();
        let mut mock_run_hyps = MockRunHyps::new();

        mock_run_hyps
            .expect_run_hyps()
            .returning(|_, _, _: &MockHypSessionBridge<RustBridge>, _| Ok(()));

        mock_run_hyps
            .expect_run_hyp()
            .returning(|_, _, _, _: &MockHypSessionBridge<RustBridge>| Ok(()));

        Self {
            run_hyps: mock_run_hyps,
            hyp_session_bridge: mock_hyp_session_bridge,
            cancellation: Cancellation::default()
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
            cancellation: self.cancellation
        }
    }

    pub fn with_runner_from_setup(self, setup: &TestDataSetup) -> HandleHypRunTrigger<HypRunner, _THypSessionBridge>
    {
        let target = env::var("HOST").expect("expected 'HOST' environment target triple");

        let runner = HypRunner::new(
            target,
            setup.workspace_path().clone(),
            setup.output_path().clone(),
            setup.coverage_path().clone()
        );

        HandleHypRunTrigger::<HypRunner, _THypSessionBridge> {
            run_hyps: runner,
            hyp_session_bridge: self.hyp_session_bridge,
            cancellation: self.cancellation
        }
    }
}

impl<TRunHyps, THypSessionBridge> HandleHypRunTrigger<TRunHyps, THypSessionBridge>
where
    TRunHyps: RunHyps,
    THypSessionBridge: HypSessionBridge<RustBridge>
{
    pub fn call(&mut self, request: HypRunRequest<RustBridge>)
    {
        handle_hyp_run_trigger(
            &mut self.run_hyps,
            &mut self.hyp_session_bridge,
            request,
            self.cancellation.clone()
        );
    }
}
