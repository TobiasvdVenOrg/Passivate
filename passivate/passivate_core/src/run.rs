use std::env;
use std::ffi::OsString;
use std::sync::OnceLock;

use camino::Utf8PathBuf;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_hyp_model::change_event::ChangeEvent;
use passivate_hyp_model::hyp_run_events::HypRunEvent;
use crate::coverage::CoverageStatus;
use crate::passivate_args::PassivateArgs;
use crate::passivate_grcov::Grcov;
use crate::test_execution::{TestRunHandler, TestRunner, change_event_thread, test_run_thread};
use passivate_delegation::{Rx, Tx};
use passivate_hyp_model::test_run::{TestRun, TestRunState};
use passivate_log::log_message::LogMessage;
use passivate_log::tx_log::TxLog;
use passivate_notify::notify_change_events::NotifyChangeEvents;

use crate::startup_errors::*;

static LOGGER: OnceLock<TxLog> = OnceLock::new();

pub struct PassivateCore
{
    pub passivate_path: Utf8PathBuf,
    pub change_event_tx: Tx<ChangeEvent>,
    pub configuration: ConfigurationManager,
    pub log_rx: Rx<LogMessage>,
    pub hyp_run_rx: Rx<HypRunEvent>,
    pub coverage_rx: Rx<CoverageStatus>,
    pub test_run: TestRun
}

pub fn run(args: PassivateArgs, main_loop: impl FnOnce(PassivateCore) -> Result<(), StartupError>) -> Result<(), StartupError>
{
    let log_rx = initialize_logger()?;

    // Channels
    let (hyp_run_tx, hyp_run_rx) = Tx::new();
    let (coverage_tx, coverage_rx) = Tx::new();
    let (configuration_tx, _configuration_rx1) = Tx::new();
    let (test_run_tx, test_run_rx) = Tx::new();
    let (change_event_tx, change_event_rx) = Tx::new();

    // Paths
    let working_dir = Utf8PathBuf::from_path_buf(env::current_dir()?).map_err(|error| StartupError::Utf8(format!("working directory was not utf8: {error:?}")))?;
    let workspace_path = args.manifest_directory.unwrap_or(working_dir);
    let passivate_path = workspace_path.join("..").join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let target_path = passivate_path.join("target");
    let binary_path = target_path.join("x86_64-pc-windows-msvc/debug");

    // Model
    let target = OsString::from("x86_64-pc-windows-msvc");

    let test_run = TestRun::from_state(TestRunState::FirstRun);
    let test_runner = TestRunner::new(
        target,
        workspace_path.clone(),
        target_path.clone(),
        coverage_path.clone()
    );

    let coverage = Grcov::builder()
        .workspace_path(workspace_path.clone())
        .output_path(coverage_path)
        .binary_path(binary_path)
        .build();

    let configuration = ConfigurationManager::from_file(configuration_tx, ".config/passivate.toml")?;

    let test_run_handler = TestRunHandler::builder()
        .configuration(configuration.clone())
        .coverage(Box::new(coverage))
        .hyp_run_tx(hyp_run_tx)
        .coverage_status_sender(coverage_tx)
        .runner(test_runner)
        .build();

    let test_run_thread = test_run_thread(test_run_rx, test_run_handler);
    let change_event_thread = change_event_thread(change_event_rx, test_run_tx);

    // Send an initial change event to trigger the first test run
    change_event_tx.send(ChangeEvent::DefaultRun);

    // Notify
    let mut change_events = NotifyChangeEvents::new(&workspace_path, change_event_tx.clone())?;

    main_loop(PassivateCore {
        passivate_path,
        change_event_tx,
        configuration,
        log_rx,
        hyp_run_rx,
        coverage_rx,
        test_run
    })?;

    let _ = change_events.stop();
    drop(change_events);
    let _ = change_event_thread.join();
    let _ = test_run_thread.join();

    Ok(())
}

fn initialize_logger() -> Result<Rx<LogMessage>, StartupError>
{
    let (log_tx, log_rx) = Tx::new();
    LOGGER
        .set(TxLog::new(log_tx))
        .map_err(|_| StartupError::LoggerAlreadyInitialized)?;
    let logger: &'static TxLog = LOGGER.get().unwrap();
    log::set_logger(logger)
        .map(|()| {
            log::set_max_level(log::LevelFilter::Info);
        })
        .map_err(StartupError::Logger)?;

    Ok(log_rx)
}
