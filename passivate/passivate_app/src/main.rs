mod app;
mod error_app;
mod startup_errors;
mod views;
mod passivate_notify;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::channel;
use std::thread;
use app::App;
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_notify::NotifyChangeEvents;
use passivate_core::test_execution::TestRunner;
use crate::error_app::ErrorApp;
use crate::startup_errors::*;

fn main() {
    match run_from_args() {
        Ok(_) => {
            println!("Exiting...");
        }
        Err(error) => {
            run_app(ErrorApp::boxed(error));
        }
    }
}

fn run_from_args() -> Result<(), StartupError> {
    let path = get_path_arg()?;

    run_from_path(&path)
}

fn get_path_arg() -> Result<PathBuf, MissingArgumentError> {
    let path = std::env::args().nth(1);

    match path {
        Some(p) => Ok(PathBuf::from(p)),
        None => Err(MissingArgumentError { argument: "path".to_string() })
    }
}

fn run_from_path(path: &Path) -> Result<(), StartupError> {
    let (change_event_sender, change_event_receiver) = channel();

    change_event_sender.send(ChangeEvent {})?;

    let change_events = NotifyChangeEvents::new(path, change_event_sender)?;

    let (tests_status_sender, tests_status_receiver) = channel();
    let mut test_execution = TestRunner::new(path, tests_status_sender);

    let exit_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let change_events_thread = thread::spawn({
        let exit_flag = exit_flag.clone();
        move || {
            while !exit_flag.load(SeqCst) {
                if let Ok(change_event) = change_event_receiver.recv() {
                    test_execution.handle_event(change_event);
                }
            }
        }
    });

    run_app(App::boxed(tests_status_receiver, change_events));

    exit_flag.store(true, SeqCst);

    change_events_thread.join().unwrap();

    Ok(())
}

fn run_app(app: Box<dyn eframe::App>) {
    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|_cc| {
            Ok(app)
        }),
    ).expect("Failed to start Passivate!");
}
