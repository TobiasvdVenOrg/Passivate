use std::ffi::OsString;
use std::sync::OnceLock;
use std::thread::JoinHandle;
use std::{env, thread};

use camino::Utf8PathBuf;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_configuration::configuration_source::FileConfigurationSource;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::Grcov;
use passivate_delegation::{Rx, Tx};
use passivate_log::log_message::LogMessage;
use passivate_log::tx_log::{LogMessageTx, TxLog};
use passivate_model_bridge::hyp_run_bridge::{HypRunOptions, HypRunTrigger, HypRunTriggerKind};
use passivate_model_bridge::hyp_session_bridge::HypSessionTrigger;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_rust::RustBridge;
use passivate_notify::notify_change_events::NotifyChangeEvents;
use passivate_run_rust::change_event_handler::change_event_thread;
use passivate_run_rust::hyp_run_handler::hyp_run_thread;
use passivate_run_rust::hyp_runner::HypRunner;

use crate::passivate_args::PassivateArgs;
use crate::passivate_state::PassivateState;
use crate::startup_errors::*;

static LOGGER: OnceLock<TxLog<crossbeam_channel::Sender<LogMessage>>> = OnceLock::new();

pub struct PassivateCore
{
    pub session: HypSession<RustBridge>,
    pub state: PassivateState<RustBridge>,
    pub passivate_path: Utf8PathBuf,
    pub session_trigger_tx: crossbeam_channel::Sender<HypSessionTrigger>,
    pub session_trigger_rx: crossbeam_channel::Receiver<HypSessionTrigger>,
    pub session_event_rx: crossbeam_channel::Receiver<HypSessionEvent<RustBridge>>,
    pub configuration: ConfigurationManager,
    pub log_rx: crossbeam_channel::Receiver<LogMessage>,
    change_events: NotifyChangeEvents
}

impl PassivateCore
{
    pub fn stop(mut self)
    {
        _ = self.change_events.stop();
    }
}

pub fn compose<'scope, 'env>(
    args: PassivateArgs,
    scope: &'scope thread::Scope<'scope, 'env>
) -> Result<PassivateCore, StartupError>
{
    let log_rx = initialize_logger()?;

    // Send requests to the session to re-run
    let (session_trigger_tx, session_trigger_rx) = crossbeam_channel::unbounded();

    // Send status of hyp run to the session
    let (session_event_tx, session_event_rx) = crossbeam_channel::unbounded();

    // Send request to run hyps to delegator (which forwards to hyp runner and cancels in-progress runs)
    let (hyp_run_request_tx, hyp_run_request_rx) = crossbeam_channel::unbounded();

    // Send requests to run hyps to hyp runner directly
    let (hyp_run_tx, hyp_run_rx) = crossbeam_channel::unbounded();

    // Paths
    let working_dir = Utf8PathBuf::from_path_buf(env::current_dir()?)
        .map_err(|error| StartupError::Utf8(format!("working directory was not utf8: {error:?}")))?;
    let workspace_path = args.manifest_directory.unwrap_or(working_dir);
    let passivate_path = workspace_path.join("..").join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let target_path = passivate_path.join("target");
    let binary_path = target_path.join("x86_64-pc-windows-msvc/debug");

    // Model
    let target = OsString::from("x86_64-pc-windows-msvc");

    let hyp_runner = HypRunner::new(target, workspace_path.clone(), target_path.clone(), coverage_path.clone());

    let configuration = ConfigurationManager::from_source(FileConfigurationSource::from(".config/passivate.toml"))?;

    hyp_run_thread(scope, hyp_run_rx, session_event_tx, hyp_runner);
    change_event_thread(scope, hyp_run_request_rx, hyp_run_tx);

    // Send an initial change event to trigger the first test run
    hyp_run_request_tx
        .send(HypRunTrigger {
            kind: HypRunTriggerKind::All,
            options: HypRunOptions::default()
        })
        .expect("startup 'hyp_run_request_tx' send failed");

    // Notify
    let change_events = NotifyChangeEvents::new(&workspace_path, session_trigger_tx.clone())?;

    let session = HypSession::new();
    let state = PassivateState::new();

    Ok(PassivateCore {
        session,
        state,
        passivate_path,
        session_trigger_tx,
        session_trigger_rx,
        session_event_rx,
        configuration,
        log_rx,
        change_events
    })
}

fn initialize_logger() -> Result<crossbeam_channel::Receiver<LogMessage>, StartupError>
{
    let (log_tx, log_rx) = crossbeam_channel::unbounded();

    LOGGER
        .set(TxLog::new(log_tx))
        .map_err(|_| StartupError::LoggerAlreadyInitialized)?;

    let logger = LOGGER.get().unwrap();

    log::set_logger(logger)
        .map(|()| {
            log::set_max_level(log::LevelFilter::Info);
        })
        .map_err(StartupError::Logger)?;

    Ok(log_rx)
}
