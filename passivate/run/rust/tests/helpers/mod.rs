use std::ffi::os_str::OsString;

use passivate_configuration::configuration::PassivateConfiguration;
use passivate_configuration::configuration_event::ConfigurationEvent;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::Grcov;
use passivate_delegation::Tx;
use passivate_hyp_model::hyp_session_event::HypSessionEvent;
use passivate_run_rust::hyp_run_handler::HypRunHandler;
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
    #[cfg(target_os = "windows")]
    let target = OsString::from("x86_64-pc-windows-msvc");

    #[cfg(target_os = "linux")]
    let target = OsString::from("aarch64-unknown-linux-gnu");

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
    #[builder(default = Tx::stub())] hyp_run_tx: Tx<HypSessionEvent>,
    #[builder(default = Tx::stub())] coverage_tx: Tx<CoverageStatus>,
    #[builder(default = Tx::stub())] configuration_tx: Tx<ConfigurationEvent>
) -> HypRunHandler
{
    let runner = test_hyp_runner(setup).call();

    let grcov = test_grcov(setup).call();

    let configuration = ConfigurationManager::new(
        PassivateConfiguration {
            coverage_enabled,
            snapshot_directories: setup.snapshot_directories()
        },
        configuration_tx
    );

    HypRunHandler::builder()
        .runner(runner)
        .coverage(Box::new(grcov))
        .hyp_run_tx(hyp_run_tx)
        .coverage_tx(coverage_tx)
        .configuration(configuration)
        .build()
}
