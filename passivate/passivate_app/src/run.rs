use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use egui::Context;
use passivate_core::actors::Actor;
use passivate_core::change_events::ChangeEvent;
use passivate_core::configuration::{ConfigurationHandler, PassivateConfig, TestRunnerImplementation};
use passivate_core::cross_cutting::ChannelLog;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::{build_test_output_parser, ChangeEventHandler, TestRunHandler, TestRunProcessor, TestRunner};
use passivate_core::test_run_model::{Snapshots, TestRun, TestRunState};
use views::{CoverageView, TestRunView};
use crate::app::App;
use crate::error_app::ErrorApp;
use crate::passivate_notify::NotifyChangeEvents;
use crate::views::{ConfigurationView, DetailsView, LogView};
use crate::{startup_errors::*, views};
use crossbeam_channel;

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
    let (configuration_sender, configuration_receiver) = crossbeam_channel::unbounded();
    let (log_sender, log_receiver) = channel();
    let (details_sender, details_receiver) = channel();

    let workspace_path = path.to_path_buf();
    let passivate_path = workspace_path.join("..").join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let target_path = passivate_path.join("target");
    let binary_path = target_path.join("x86_64-pc-windows-msvc/debug");
    
    let configuration = PassivateConfig::default();

    let log = ChannelLog::new(log_sender);

    let target = OsString::from("x86_64-pc-windows-msvc");
    let test_runner = TestRunner::new(target, workspace_path.clone(), target_path.clone(), coverage_path.clone(), Box::new(log.clone()));
    let parser = build_test_output_parser(&TestRunnerImplementation::Nextest);
    let test_run = TestRun::from_state(TestRunState::FirstRun);
    let test_processor = TestRunProcessor::from_test_run(Box::new(test_runner), parser, test_run, Box::new(log.clone()));
    let coverage = Grcov::new(&workspace_path, &coverage_path, &binary_path);

    let test_run_handler = TestRunHandler::new(test_processor, Box::new(coverage), tests_status_sender, coverage_sender, Box::new(log.clone()), configuration.coverage_enabled);
    let mut test_run_actor = Actor::new(test_run_handler);

    let change_handler = ChangeEventHandler::new(test_run_actor.api(), Box::new(log.clone()));
    let mut change_actor = Actor::new(change_handler);
    
    let configuration_handler = ConfigurationHandler::new(change_actor.api(), configuration_sender);
    let mut configuration_actor = Actor::new(configuration_handler);

    let mut change_events = NotifyChangeEvents::new(path, change_actor.api())?;

    let tests_view = TestRunView::new(tests_status_receiver, details_sender);

    let hacky_snapshots = Snapshots::new(workspace_path.join("passivate_app").join("tests").join("snapshots"));
    let mut details_view = DetailsView::new(details_receiver, change_actor.api(), configuration_receiver.clone());
    details_view.set_snapshots(hacky_snapshots);

    let coverage_view = CoverageView::new(coverage_receiver, configuration_actor.api());
    let configuration_view = ConfigurationView::new(configuration_actor.api(), configuration_receiver.clone(), configuration);
    let log_view = LogView::new(log_receiver);

    // Send an initial change event to trigger the first test run
    change_actor.api().send(ChangeEvent::File);

    run_app(Box::new(App::new(tests_view, details_view, coverage_view, configuration_view, log_view)), context_accessor)?;

    test_run_actor.stop();
    configuration_actor.stop();
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
