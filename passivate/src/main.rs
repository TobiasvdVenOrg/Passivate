mod app;
mod egui_tests_view;
mod error_app;
mod startup_errors;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use app::App;
use eframe::CreationContext;
use passivate_core::change_events::AsyncChangeEventHandler;
use passivate_core::passivate_notify::NotifyChangeEvents;
use passivate_core::test_execution::{TestRunner, TestsStatus};
use crate::egui_tests_view::EguiTestsView;
use crate::error_app::ErrorApp;
use crate::startup_errors::*;

fn get_path_arg() -> Result<PathBuf, MissingArgumentError> {
    let path = std::env::args().nth(1);

    match path {
        Some(p) => Ok(PathBuf::from(p)),
        None => Err(MissingArgumentError { argument: "path".to_string() })
    }
}
fn build_app(cc: &CreationContext) -> Result<Box<dyn eframe::App>, StartupError> {
    let path = get_path_arg()?;

    let tests_status = Arc::new(RwLock::new(TestsStatus::waiting()));
    let tests_view = EguiTestsView::new(cc.egui_ctx.clone(), tests_status.clone());
    let test_execution = TestRunner::new(Box::new(tests_view));
    let change_event_handler = AsyncChangeEventHandler::new(Box::new(test_execution));
    let change_events = NotifyChangeEvents::new(&path, change_event_handler)?;

    Ok(App::boxed(tests_status, change_events))
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
