use std::env;
use std::sync::OnceLock;

use camino::Utf8PathBuf;
use passivate_configuration::configuration::ConfigurationChange;
use passivate_configuration::configuration_errors::ConfigurationError;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_configuration::configuration_source::FileConfigurationSource;
use passivate_configuration::default_paths::{self, DefaultPaths};
use passivate_log::log_message::LogMessage;
use passivate_log::tx_log::TxLog;
use passivate_model_bridge::hyp_run_request::HypRunRequest;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_bridge::source_change_event::SourceChangeEvent;
use passivate_model_core::hyp_session::HypSession;
use passivate_notify::notify_change_events::NotifyChangeEvents;
use passivate_run_rust::hyp_run_handler;
use passivate_run_rust::hyp_runner::HypRunner;
use passivate_run_rust::model::RustBridge;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

use crate::passivate_args::PassivateArgs;
use crate::passivate_state::PassivateState;
use crate::startup_errors::*;

static LOGGER: OnceLock<TxLog<crossbeam_channel::Sender<LogMessage>>> = OnceLock::new();

pub struct PassivateCore
{
    pub session: HypSession<RustBridge>,
    pub state: PassivateState<RustBridge>,
    pub source_change_rx: crossbeam_channel::Receiver<SourceChangeEvent>,
    pub hyp_run_tx: tokio::sync::mpsc::UnboundedSender<HypRunRequest<RustBridge>>,
    pub session_event_rx: crossbeam_channel::Receiver<HypSessionEvent<RustBridge>>,
    pub configuration: ConfigurationManager,
    pub log_rx: crossbeam_channel::Receiver<LogMessage>,
    change_events: NotifyChangeEvents,
    pub hyp_run_task: Option<JoinHandle<()>>
}

impl PassivateCore
{
    pub fn stop(mut self)
    {
        _ = self.change_events.stop();
    }
}

pub fn compose(args: PassivateArgs, runtime: &Runtime) -> Result<PassivateCore, StartupError>
{
    let log_rx = initialize_logger()?;

    // Send source changes, which may trigger hyp re-runs
    let (source_change_tx, source_change_rx) = crossbeam_channel::unbounded();

    // Send status of hyp run to the session
    let (session_event_tx, session_event_rx) = crossbeam_channel::unbounded();

    // Send requests to run hyps
    let (hyp_run_tx, hyp_run_rx) = tokio::sync::mpsc::unbounded_channel();

    // Paths
    let working_dir = Utf8PathBuf::from_path_buf(env::current_dir()?)
        .map_err(|error| StartupError::Utf8(format!("working directory was not utf8: {error:?}")))?;
    let default_paths = DefaultPaths::new(working_dir);
    let root_path = args.root_directory.unwrap_or(default_paths.root.clone());

    let hyp_runner = HypRunner;

    let mut configuration =
        ConfigurationManager::from_source(FileConfigurationSource::from(".config/passivate.toml"), default_paths)
            .map_err(ConfigurationError::Load)?;

    configuration
        .change(ConfigurationChange::TargetDir(args.target_directory))
        .map_err(ConfigurationError::Persist)?;

    let hyp_run_task = hyp_run_handler::spawn_hyp_run_future(runtime, hyp_run_rx, session_event_tx, hyp_runner);

    // Notify
    let change_events = NotifyChangeEvents::start_watching(root_path, source_change_tx)?;

    let session = HypSession::new();
    let state = PassivateState::new();

    Ok(PassivateCore {
        session,
        state,
        source_change_rx,
        hyp_run_tx,
        session_event_rx,
        configuration,
        log_rx,
        change_events,
        hyp_run_task: Some(hyp_run_task)
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
            log::set_max_level(log::LevelFilter::max());
        })
        .map_err(StartupError::Logger)?;

    Ok(log_rx)
}
