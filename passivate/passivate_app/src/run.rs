use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::{fs, thread};
use egui::Context;
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::passivate_cargo::CargoTest;
use passivate_core::passivate_grcov::Grcov;
use passivate_core::test_execution::TestRunner;
use views::{CoverageView, TestsStatusView};
use crate::app::App;
use crate::error_app::ErrorApp;
use crate::passivate_notify::NotifyChangeEvents;
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
    let (change_event_sender, change_event_receiver) = channel();
    let exit_event_sender = change_event_sender.clone();

    // Send an initial change event to trigger an immediate test run
    change_event_sender.send(ChangeEvent::File)?;

    let mut change_events = NotifyChangeEvents::new(path, change_event_sender)?;

    let (tests_status_sender, tests_status_receiver) = channel();
    let (coverage_sender, coverage_receiver) = channel();

    let workspace_path = path.to_path_buf();
    let passivate_path = workspace_path.join(".passivate");
    let coverage_path = passivate_path.join("coverage");
    let binary_path = Path::new("./target/x86_64-pc-windows-msvc/debug/");

    fs::create_dir_all(&coverage_path)?; 

    // Absolute dir, because a relative dir will cause profraw files to be output relative to each individual project in the workspace
    let profraw_output_path = fs::canonicalize(&coverage_path)?;

    let change_events_thread = thread::spawn(move || {
        let cargo_test = CargoTest::new(&workspace_path, &profraw_output_path);
        let coverage = Grcov::new(&workspace_path, &coverage_path, binary_path);
        let mut test_execution = TestRunner::new(Box::new(cargo_test), Box::new(coverage), tests_status_sender, coverage_sender);

        while let Ok(change_event) = change_event_receiver.recv() {
            match change_event {
                ChangeEvent::File => test_execution.handle_event(change_event),
                ChangeEvent::Exit => break,
            }
        }
    });

    let tests_view = TestsStatusView::new(tests_status_receiver);
    let coverage_view = CoverageView::new(coverage_receiver);
    run_app(Box::new(App::new(tests_view, coverage_view)), context_accessor)?;

    exit_event_sender.send(ChangeEvent::Exit)?;

    let _ = change_events.stop();
    change_events_thread.join().unwrap();

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
