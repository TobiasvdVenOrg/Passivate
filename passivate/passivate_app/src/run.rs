use std::ffi::OsString;

use camino::{Utf8Path, Utf8PathBuf};
use egui::Context;
use passivate_core::change_events::ChangeEvent;
use passivate_core::configuration::{ConfigurationManager, PassivateConfig};
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::{change_event_thread, test_run_thread, TestRunHandler, TestRunner};
use passivate_core::test_run_model::{TestRun, TestRunState};
use passivate_delegation::Tx;
use passivate_notify::notify_change_events::NotifyChangeEvents;
use views::{CoverageView, TestRunView};

use crate::app::App;
use crate::error_app::ErrorApp;
use crate::startup_errors::*;
use crate::views;
use crate::views::{ConfigurationView, DetailsView, LogView};

pub fn run(context_accessor: Box<dyn FnOnce(Context)>) -> Result<(), StartupError>
{
    match get_path_arg()
    {
        Ok(path) => run_from_path(&path, context_accessor),
        Err(error) => run_app(ErrorApp::boxed(error.into()), context_accessor)
    }
}

pub fn get_path_arg() -> Result<Utf8PathBuf, MissingArgumentError>
{
    let path = std::env::args().nth(1);

    match path
    {
        Some(p) => Ok(Utf8PathBuf::from(p)),
        None => Err(MissingArgumentError { argument: "path".to_string() })
    }
}

pub fn run_from_path(path: &Utf8Path, context_accessor: Box<dyn FnOnce(Context)>) -> Result<(), StartupError>
{
    // Channels
    let (tests_status_tx, tests_status_rx) = Tx::new();
    let (coverage_tx, coverage_rx) = Tx::new();
    let (configuration_tx, _configuration_rx1) = Tx::new();
    let (log_tx, log_rx) = Tx::new();
    let (details_tx, details_rx) = Tx::new();
    let (test_run_tx, test_run_rx) = Tx::new();
    let (change_event_tx, change_event_rx) = Tx::new();

    // Paths
    let workspace_path = path.to_path_buf();
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
        coverage_path.clone(),
        test_run);


    let coverage = Grcov::builder()
        .workspace_path(workspace_path)
        .output_path(coverage_path)
        .binary_path(binary_path)
        .build();
    
    let configuration = ConfigurationManager::new(PassivateConfig::default(), configuration_tx);
    let test_run_handler = TestRunHandler::builder()
        .configuration(configuration.clone())
        .coverage(Box::new(coverage))
        .tests_status_sender(tests_status_tx)
        .coverage_status_sender(coverage_tx)
        .log(log_tx)
        .runner(test_runner)
        .build();

    let test_run_thread = test_run_thread(test_run_rx, test_run_handler);
    let change_event_thread = change_event_thread(change_event_rx, test_run_tx);

    // Send an initial change event to trigger the first test run
    change_event_tx.send(ChangeEvent::DefaultRun);

    // Notify
    let mut change_events = NotifyChangeEvents::new(path, change_event_tx.clone())?;

    // Views
    let tests_view = TestRunView::new(tests_status_rx, details_tx);
    let details_view = DetailsView::new(details_rx, change_event_tx.clone(), configuration.clone());
    let coverage_view = CoverageView::new(coverage_rx, configuration.clone());
    let configuration_view = ConfigurationView::new(configuration, change_event_tx);
    let log_view = LogView::new(log_rx);

    // Block until app closes
    run_app(Box::new(App::new(tests_view, details_view, coverage_view, configuration_view, log_view)), context_accessor)?;

    let _ = change_events.stop();
    drop(change_events);
    let _ = change_event_thread.join();
    let _ = test_run_thread.join();

    Ok(())
}

pub fn run_app(app: Box<dyn eframe::App>, context_accessor: Box<dyn FnOnce(Context)>) -> Result<(), StartupError>
{
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position([1920.0, 0.0])
            .with_inner_size([1024.0, 512.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| {
            context_accessor(cc.egui_ctx.clone());

            Ok(app)
        })
    )
    .expect("Failed to start Passivate!");

    Ok(())
}
