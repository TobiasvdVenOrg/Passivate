use std::env;
use std::ffi::os_str::OsString;

use passivate_configuration::configuration::PassivateConfiguration;
use passivate_configuration::configuration_event::ConfigurationEvent;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::Grcov;
use passivate_delegation::{Cancellation, Tx};
use passivate_model_bridge::hyp_run_bridge::HypRunTrigger;
use passivate_model_bridge::hyp_session_bridge::{HypSessionBridge, MockHypSessionBridge};
use passivate_model_core::hyp_session_event::HypSessionEvent;
use passivate_model_rust::RustBridge;
use passivate_run_core::session_event_tx::SessionEventTx;
use passivate_run_rust::hyp_run_handler::{HypRunHandler, handle_hyp_run_trigger};
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

#[bon::builder]
pub fn test_hyp_run_handler(
    #[builder(start_fn)] setup: &TestDataSetup,
    #[builder(default = false)] coverage_enabled: bool,
    #[builder(default = SessionEventTx::stub())] hyp_run_tx: SessionEventTx<RustBridge>,
    #[builder(default = Tx::stub())] coverage_tx: Tx<CoverageStatus>,
    #[builder(default = Tx::stub())] configuration_tx: Tx<ConfigurationEvent>
) -> HypRunHandler
{
    let runner = test_hyp_runner(setup).call();

    let grcov = test_grcov(setup).call();

    let configuration = ConfigurationManager::new(PassivateConfiguration {
        coverage_enabled,
        snapshot_directories: setup.snapshot_directories()
    });

    HypRunHandler::builder()
        .runner(runner)
        .coverage(Box::new(grcov))
        .hyp_run_tx(hyp_run_tx)
        .coverage_tx(coverage_tx)
        .configuration(configuration)
        .build()
}

pub struct HandleHypRunTrigger<THypSessionBridge>
where
    THypSessionBridge: HypSessionBridge
{
    runner: HypRunner,
    hyp_session_bridge: THypSessionBridge,
    cancellation: Cancellation
}

impl HandleHypRunTrigger<MockHypSessionBridge>
{
    pub fn new(setup: &TestDataSetup) -> Self
    {
        let mut mock_hyp_session_bridge = MockHypSessionBridge::new();
        mock_hyp_session_bridge.expect_request_rerun();

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

impl<THypSessionBridge> HandleHypRunTrigger<THypSessionBridge>
where
    THypSessionBridge: HypSessionBridge
{
    pub fn call(mut self, trigger: HypRunTrigger<RustBridge>)
    {
        handle_hyp_run_trigger(
            &mut self.runner,
            &mut self.hyp_session_bridge,
            trigger,
            self.cancellation.clone()
        );
    }
}
