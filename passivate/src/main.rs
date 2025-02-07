mod app;
mod error_app;
mod startup_errors;
pub mod passivate_notify;

use std::path::PathBuf;
use std::sync::mpsc::channel;
use app::App;
use eframe::CreationContext;
use passivate_core::change_events::{AsyncChangeEventHandler, ChangeEvent};
use passivate_notify::NotifyChangeEvents;
use passivate_core::test_execution::{TestRunner};
use crate::error_app::ErrorApp;
use crate::startup_errors::*;

fn get_path_arg() -> Result<PathBuf, MissingArgumentError> {
    let path = std::env::args().nth(1);

    match path {
        Some(p) => Ok(PathBuf::from(p)),
        None => Err(MissingArgumentError { argument: "path".to_string() })
    }
}
fn build_app(_cc: &CreationContext) -> Result<Box<dyn eframe::App>, StartupError> {
    let path = get_path_arg()?;

    let (change_event_sender, change_event_receiver) = channel();

    let _ = change_event_sender.send(ChangeEvent {});

    let change_events = NotifyChangeEvents::new(&path, change_event_sender)?;

    let (tests_status_sender, tests_status_receiver) = channel();
    let test_execution = TestRunner::new(&path, tests_status_sender);
    let _change_event_handler = AsyncChangeEventHandler::new(Box::new(test_execution), change_event_receiver);

    Ok(App::boxed(tests_status_receiver, change_events))
}

fn main() {
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
            let result = build_app(cc);

            match result {
                Ok(app) => Ok(app),
                Err(error) => {
                    Ok(ErrorApp::boxed(error))
                }
            }
        }),
    ).expect("failed to open window");
}
