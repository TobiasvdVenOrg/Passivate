use std::env;

use passivate_coverage::grcov::Grcov;
use passivate_delegation::{Cancellation, cancellation};
use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_bridge::{self, MockHypSessionBridge};
use passivate_model_rust::RustBridge;
use passivate_run_rust::hyp_run_handler::{HypSessionBridge, handle_hyp_run_trigger};
use passivate_run_rust::hyp_runner::HypRunner;
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

pub struct HandleHypRunTrigger<THypSessionBridge>
where
    THypSessionBridge: HypSessionBridge<RustBridge>
{
    runner: HypRunner,
    hyp_session_bridge: THypSessionBridge,
    cancellation: Cancellation
}

impl HandleHypRunTrigger<MockHypSessionBridge<RustBridge>>
{
    pub fn new(setup: &TestDataSetup) -> Self
    {
        let mock_hyp_session_bridge = MockHypSessionBridge::new();

        let target = env::var("HOST").expect("expected 'HOST' environment target triple");

        let runner = HypRunner::new(
            target,
            setup.workspace_path().clone(),
            setup.output_path().clone(),
            setup.coverage_path().clone()
        );

        Self {
            runner,
            hyp_session_bridge: mock_hyp_session_bridge,
            cancellation: Cancellation::default()
        }
    }
}

impl<THypSessionBridge_> HandleHypRunTrigger<THypSessionBridge_>
where
    THypSessionBridge_: HypSessionBridge<RustBridge>
{
    pub fn with_hyp_session_bridge<THypSessionBridge>(
        self,
        hyp_session_bridge: THypSessionBridge
    ) -> HandleHypRunTrigger<THypSessionBridge>
    where
        THypSessionBridge: HypSessionBridge<RustBridge>
    {
        HandleHypRunTrigger {
            runner: self.runner,
            hyp_session_bridge,
            cancellation: self.cancellation
        }
    }
}

impl<THypSessionBridge> HandleHypRunTrigger<THypSessionBridge>
where
    THypSessionBridge: HypSessionBridge<RustBridge>
{
    pub fn call(&mut self, request: HypRunRequest<RustBridge>)
    {
        handle_hyp_run_trigger(
            &mut self.runner,
            &mut self.hyp_session_bridge,
            request,
            self.cancellation.clone()
        );
    }
}
