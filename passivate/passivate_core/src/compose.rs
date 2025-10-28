use std::env;
use std::ffi::OsString;
use std::sync::OnceLock;
use std::thread::JoinHandle;

use camino::Utf8PathBuf;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::Grcov;
use passivate_hyp_execution::change_event_handler::change_event_thread;
use passivate_hyp_execution::test_run_handler::{test_run_thread, TestRunHandler};
use passivate_hyp_execution::hyp_runner::HypRunner;
use passivate_hyp_model::hyp_run_trigger::HypRunTrigger;
use crate::passivate_args::PassivateArgs;
use crate::passivate_state::PassivateState;
use passivate_delegation::{Rx, Tx};
use passivate_log::log_message::LogMessage;
use passivate_log::tx_log::TxLog;
use passivate_notify::notify_change_events::NotifyChangeEvents;

use crate::startup_errors::*;

static LOGGER: OnceLock<TxLog> = OnceLock::new();

pub struct PassivateCore
{
    pub state: PassivateState,
    pub passivate_path: Utf8PathBuf,
    pub change_event_tx: Tx<HypRunTrigger>,
    pub configuration: ConfigurationManager,
    pub log_rx: Rx<LogMessage>,
    pub coverage_rx: Rx<CoverageStatus>,
    change_events: NotifyChangeEvents,
    change_event_thread: JoinHandle<()>,
    test_run_thread: JoinHandle<TestRunHandler>
}

impl PassivateCore
{
    pub fn stop(mut self)
    {
        _ = self.change_events.stop();
        _ = self.change_event_thread.join();
        _ = self.test_run_thread.join();
    }
}

pub fn compose(args: PassivateArgs) -> Result<PassivateCore, StartupError>
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

    let test_runner = HypRunner::new(
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
    change_event_tx.send(HypRunTrigger::DefaultRun);

    // Notify
    let change_events = NotifyChangeEvents::new(&workspace_path, change_event_tx.clone())?;

    let state = PassivateState::new(hyp_run_rx);

    Ok(PassivateCore {
        state,
        passivate_path,
        change_event_tx,
        configuration,
        log_rx,
        coverage_rx,
        change_events,
        change_event_thread,
        test_run_thread
    })
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
