use std::sync::mpsc::channel;

use egui_kittest::Harness;
use crate::views::{TestsStatusView, View};
use passivate_core::test_execution::{CompleteTestsStatus, TestsStatus};
use stdext::function_name;

#[test]
pub fn tests_status_view_shows_when_tests_are_running() {
    run_and_snapshot(TestsStatus::Running, &test_name(function_name!()));
}

#[test]
pub fn tests_status_view_shows_when_no_tests_were_found() {
    let completed = CompleteTestsStatus { tests: vec!() };

    run_and_snapshot(TestsStatus::Completed(completed), &test_name(function_name!()));
}

fn run_and_snapshot(tests_status: TestsStatus, snapshot_name: &str) {
    let (sender, receiver)  = channel();
    let mut tests_status_view = TestsStatusView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        tests_status_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    sender.send(tests_status).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(snapshot_name);
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}