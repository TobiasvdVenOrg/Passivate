mod app;
mod egui_tests_view;

use app::App;
use notify::*;
use eframe::CreationContext;
use futures::SinkExt;
use passivate_core::change_events::AsyncChangeEventHandler;
use passivate_core::passivate_notify::NotifyChangeEvents;
use passivate_core::test_execution::TestExecution;
use passivate_core::tests_view::TestsStatus;
use crate::egui_tests_view::EguiTestsView;

fn build_app(cc: &CreationContext) -> App {
    let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");

    let tests_status = TestsStatus::new("Bladiebloe");
    let tests_view = EguiTestsView::new(cc.egui_ctx.clone(), tests_status.clone());
    let test_execution = TestExecution::new(Box::new(tests_view));
    let change_event_handler = AsyncChangeEventHandler::new(Box::new(test_execution));
    let change_events = NotifyChangeEvents::new(&path, change_event_handler);

    App::new(tests_status, Box::new(change_events))
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
            let app = build_app(cc);
            Ok(Box::new(app))
        }),
    ).expect("Unable to open window.");
}
