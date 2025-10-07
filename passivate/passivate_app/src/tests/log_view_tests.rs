use std::time::Duration;

use chrono::DateTime;
use egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use passivate_core::cross_cutting::LogEvent;
use passivate_delegation::Tx;
use passivate_hyp_names::test_name;

use crate::views::{LogView, View};

#[test]
pub fn show_a_single_log()
{
    let (tx, rx) = Tx::new();
    let mut log_view = LogView::new(rx);

    let ui = |ui: &mut egui::Ui| {
        log_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let example_log = LogEvent::new_with_timestamp("Hey, this is a log message!", DateTime::from_timestamp_nanos(1_662_921_288_000_000_000));
    tx.send(example_log);

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name!());
}

#[test]
pub fn many_logs_are_scrollable()
{
    let (tx, rx) = Tx::new();
    let mut log_view = LogView::new(rx);

    let ui = |ui: &mut egui::Ui| {
        ui.set_max_height(100.0);
        log_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);
    harness.run();
    for n in 0 .. 20
    {
        let mut timestamp = DateTime::from_timestamp_nanos(1_662_921_288_000_000_000);
        timestamp += Duration::from_secs(n);

        let example_log = LogEvent::new_with_timestamp("Hey, this is a log message!", timestamp);
        tx.send(example_log);

        harness.run();
    }

    let thing = harness.get_all_by_role(Role::Label).next().unwrap();
    thing.hover();

    harness.run();

    let scrollbar = harness.get_all_by_role(Role::Unknown).next().unwrap();
    scrollbar.hover();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name!());
}
