use std::sync::OnceLock;
use std::{env, thread};

use camino::Utf8PathBuf;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_configuration::configuration_source::FileConfigurationSource;
use passivate_delegation::CancellableMessage;
use passivate_log::log_message::LogMessage;
use passivate_log::tx_log::TxLog;
use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_bridge::source_change_event::SourceChangeEvent;
use passivate_model_core::hyp_session::HypSession;
use passivate_notify::notify_change_events::NotifyChangeEvents;
use passivate_run_rust::hyp_run_handler::hyp_run_thread;
use passivate_run_rust::hyp_runner::HypRunner;
use passivate_run_rust::model::RustBridge;

use crate::passivate_args::PassivateArgs;
use crate::passivate_state::PassivateState;
use crate::startup_errors::*;

static LOGGER: OnceLock<TxLog<crossbeam_channel::Sender<LogMessage>>> = OnceLock::new();

pub struct PassivateCore
{
    pub session: HypSession<RustBridge>,
    pub state: PassivateState<RustBridge>,
    pub passivate_path: Utf8PathBuf,
    pub source_change_rx: crossbeam_channel::Receiver<SourceChangeEvent>,
    pub hyp_run_tx: crossbeam_channel::Sender<CancellableMessage<HypRunRequest<RustBridge>>>,
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

    // Send source changes, which may trigger hyp re-runs
    let (source_change_tx, source_change_rx) = crossbeam_channel::unbounded();

    // Send status of hyp run to the session
    let (session_event_tx, session_event_rx) = crossbeam_channel::unbounded();

    // Send requests to run hyps
    let (hyp_run_tx, hyp_run_rx) = crossbeam_channel::unbounded();

    // Paths
    let working_dir = Utf8PathBuf::from_path_buf(env::current_dir()?)
        .map_err(|error| StartupError::Utf8(format!("working directory was not utf8: {error:?}")))?;
    let workspace_path = args.manifest_directory.unwrap_or(working_dir);
    let passivate_path = workspace_path.join("..").join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let target_path = passivate_path.join("target");
    let binary_path = target_path.join("x86_64-pc-windows-msvc/debug");

    let hyp_runner = HypRunner::new(workspace_path.clone(), target_path.clone(), coverage_path.clone());

    let configuration = ConfigurationManager::from_source(FileConfigurationSource::from(".config/passivate.toml"))?;

    hyp_run_thread(scope, hyp_run_rx, session_event_tx, hyp_runner);

    // Notify
    let change_events = NotifyChangeEvents::new(&workspace_path, source_change_tx)?;

    let session = HypSession::new();
    let state = PassivateState::new();

    Ok(PassivateCore {
        session,
        state,
        passivate_path,
        source_change_rx,
        hyp_run_tx,
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
