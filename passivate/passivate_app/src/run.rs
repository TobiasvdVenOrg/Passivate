use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use egui::Context;
use passivate_core::actors::Actor;
use passivate_core::change_events::ChangeEvent;
use passivate_core::configuration::{PassivateConfig, TestRunnerImplementation};
use passivate_core::cross_cutting::ChannelLog;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::{build_test_output_parser, ChangeEventHandler, TestRunProcessor, TestRunner};
use passivate_core::test_run_model::{TestRun, TestRunState};
use views::{CoverageView, TestRunView};
use crate::app::App;
use crate::error_app::ErrorApp;
use crate::passivate_notify::NotifyChangeEvents;
use crate::views::{ConfigurationView, LogView};
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
    
    let (tests_status_sender, tests_status_receiver) = channel();
    let (coverage_sender, coverage_receiver) = channel();
    let (configuration_sender, configuration_receiver) = channel();
    let (configuration_change_sender, configuration_change_receiver) = channel();
    let (log_sender, log_receiver) = channel();

    let workspace_path = path.to_path_buf();
    let passivate_path = workspace_path.join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let target_path = passivate_path.join("target");
    let binary_path = target_path.join("x86_64-pc-windows-msvc/debug");
    
    let log = ChannelLog::new(log_sender);

    let test_runner = TestRunner::new(workspace_path.clone(), target_path.clone(), coverage_path.clone(), Box::new(log.clone()));
    let parser = build_test_output_parser(&TestRunnerImplementation::Nextest);
    let test_run = TestRun::from_state(TestRunState::FirstRun);
    let test_processor = TestRunProcessor::from_test_run(Box::new(test_runner), parser, test_run, Box::new(log.clone()));
    let coverage = Grcov::new(&workspace_path, &coverage_path, &binary_path);
    let change_handler = ChangeEventHandler::new(test_processor, Box::new(coverage), tests_status_sender, coverage_sender, Box::new(log.clone()));
    let mut change_actor = Actor::new(change_handler);
    
    let mut change_events = NotifyChangeEvents::new(path, change_actor.api())?;

    let tests_view = TestRunView::new(tests_status_receiver);
    let coverage_view = CoverageView::new(coverage_receiver, change_actor.api());
    let configuration_view = ConfigurationView::new(configuration_change_sender, configuration_receiver, PassivateConfig::default());
    let log_view = LogView::new(log_receiver);

    // Send an initial change event to trigger the first test run
    change_actor.api().send(ChangeEvent::File);

    run_app(Box::new(App::new(tests_view, coverage_view, configuration_view, log_view)), context_accessor)?;

    change_actor.stop();

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
