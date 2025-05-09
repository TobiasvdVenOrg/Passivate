use std::ffi::OsString;
use std::path::{Path, PathBuf};
use egui::Context;
use passivate_delegation::{tx_1_rx_1, tx_1_rx_2, Actor, Cancellation};
use passivate_core::change_events::ChangeEvent;
use passivate_core::configuration::{ConfigurationHandler, PassivateConfig, TestRunnerImplementation};
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::{build_test_output_parser, ChangeEventHandler, TestRunActor, TestRunProcessor, TestRunner};
use passivate_core::test_run_model::{TestRun, TestRunState};
use passivate_core::cross_cutting::*;
use views::{CoverageView, TestRunView};
use crate::app::App;
use crate::error_app::ErrorApp;
use crate::passivate_notify::NotifyChangeEvents;
use crate::views::{ConfigurationView, DetailsView, LogView};
use crate::{startup_errors::*, views};

pub fn run(context_accessor: Box<dyn FnOnce(Context)>) -> Result<(), StartupError> {
    match get_path_arg() {
        Ok(path) => {
            run_from_path(&path, context_accessor)
        }
        Err(error) => {
            run_app(ErrorApp::boxed(error.into()), context_accessor)
        }
    }
}

pub fn get_path_arg() -> Result<PathBuf, MissingArgumentError> {
    let path = std::env::args().nth(1);

    match path {
        Some(p) => Ok(PathBuf::from(p)),
        None => Err(MissingArgumentError { argument: "path".to_string() })
    }
}

pub fn run_from_path(path: &Path, context_accessor: Box<dyn FnOnce(Context)>) -> Result<(), StartupError> {
    // Channels
    let (tests_status_sender, tests_status_receiver) = tx_1_rx_1();
    let (coverage_sender, coverage_receiver) = tx_1_rx_1();
    let (configuration_sender, configuration_rx1, configuration_rx2) = tx_1_rx_2();
    let (log_tx, log_rx) = tx_1_rx_1();
    let (details_sender, details_receiver) = tx_1_rx_1();

    // Paths
    let workspace_path = path.to_path_buf();
    let passivate_path = workspace_path.join("..").join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let target_path = passivate_path.join("target");
    let binary_path = target_path.join("x86_64-pc-windows-msvc/debug");
    
    let configuration = PassivateConfig::default();

    // Model
    let target = OsString::from("x86_64-pc-windows-msvc");
    let test_runner = TestRunner::new(target, workspace_path.clone(), target_path.clone(), coverage_path.clone());
    let parser = build_test_output_parser(&TestRunnerImplementation::Nextest);
    let test_run = TestRun::from_state(TestRunState::FirstRun);
    let test_processor = TestRunProcessor::from_test_run(Box::new(test_runner), parser, test_run);
    let coverage = Grcov::new(&workspace_path, &coverage_path, &binary_path);

    // Actors
    let (_test_run_actor, test_run_tx) = TestRunActor::new(test_processor, Box::new(coverage), tests_status_sender, coverage_sender, ChannelLog::boxed(log_tx), configuration.coverage_enabled);

    let change_handler = ChangeEventHandler::new(test_run_tx);
    let (_change_actor, change_actor_tx1, change_actor_tx2, change_actor_tx3) = Actor::new_3(change_handler);
    
    // Send an initial change event to trigger the first test run
    change_actor_tx1.send(ChangeEvent::File, Cancellation::default());

    let configuration_handler = ConfigurationHandler::new(change_actor_tx1.into(), configuration_sender);
    let (_configuration_actor, configuration_actor_tx1, configuration_actor_tx2) = Actor::new_2(configuration_handler);

    // Notify
    let mut change_events = NotifyChangeEvents::new(path, change_actor_tx2.into())?;

    // Views
    let tests_view = TestRunView::new(tests_status_receiver, details_sender);
    let details_view = DetailsView::new(details_receiver, change_actor_tx3.into(), configuration_rx1);
    let coverage_view = CoverageView::new(coverage_receiver, configuration_actor_tx1.into());
    let configuration_view = ConfigurationView::new(configuration_actor_tx2.into(), configuration_rx2, configuration);
    let log_view = LogView::new(log_rx);

    // Block until app closes
    run_app(Box::new(App::new(tests_view, details_view, coverage_view, configuration_view, log_view)), context_accessor)?;

    let _ = change_events.stop();

    Ok(())
}

pub fn run_app(app: Box<dyn eframe::App>, context_accessor: Box<dyn FnOnce(Context)>) -> Result<(), StartupError> {
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| {
            context_accessor(cc.egui_ctx.clone());

            Ok(app)
        }),
    ).expect("Failed to start Passivate!");

    Ok(())
}
