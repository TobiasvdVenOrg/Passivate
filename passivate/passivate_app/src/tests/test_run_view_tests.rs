use std::sync::mpsc::channel;
use egui_kittest::Harness;
use crate::views::{TestRunView, View};
use passivate_core::test_run_model::{ActiveTestRun, BuildFailedTestRun, TestRun};
use stdext::function_name;

#[test]
pub fn test_run_view_shows_when_test_run_is_starting() {
    run_and_snapshot(TestRun::Starting, &test_name(function_name!()));
}

#[test]
pub fn test_run_view_shows_when_no_tests_were_found() {
    let active = ActiveTestRun { tests: vec!() };

    run_and_snapshot(TestRun::Active(active), &test_name(function_name!()));
}

#[test]
pub fn test_run_view_shows_when_build_failed() {
    let build_failed = TestRun::BuildFailed(BuildFailedTestRun { message: "Something didn't compile!".to_string() });
    run_and_snapshot(build_failed, &test_name(function_name!()));
}

fn run_and_snapshot(tests_status: TestRun, snapshot_name: &str) {
    let (sender, receiver)  = channel();
    let mut tests_status_view = TestRunView::new(receiver);

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