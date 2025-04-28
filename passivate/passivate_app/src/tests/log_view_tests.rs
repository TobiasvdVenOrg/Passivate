use std::time::Duration;
use chrono::DateTime;
use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::Queryable};
use crate::views::{LogView, View};
use passivate_core::cross_cutting::LogEvent;
use passivate_delegation::channel;
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
    sender.send(example_log);

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn many_logs_are_scrollable() {
    let (sender, receiver)  = channel();
    let mut log_view = LogView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        ui.set_max_height(100.0);
        log_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);
    harness.run();
    for n in 0..20 {
        let mut timestamp = DateTime::from_timestamp_nanos(1_662_921_288_000_000_000);
        timestamp += Duration::from_secs(n);

        let example_log = LogEvent::new_with_timestamp("Hey, this is a log message!", timestamp);
        sender.send(example_log);

        harness.run();
    }

    let thing = harness.get_all_by_role(Role::Label).next().unwrap();
    thing.hover();

    harness.run();

    let scrollbar = harness.get_all_by_role(Role::Unknown).next().unwrap();
    scrollbar.hover();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}
