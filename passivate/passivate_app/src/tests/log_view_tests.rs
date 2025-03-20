use std::sync::mpsc::channel;
use chrono::{DateTime, Utc};
use egui_kittest::Harness;
use crate::views::{LogView, View};
use passivate_core::cross_cutting::LogEvent;
use stdext::function_name;

#[test]
pub fn show_a_single_log() {
    let (sender, receiver)  = channel();
    let mut log_view = LogView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        log_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let example_log = LogEvent::new_with_timestamp("Hey, this is a log message!", DateTime::from_timestamp_nanos(1_662_921_288_000_000_000));
    sender.send(example_log).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}
