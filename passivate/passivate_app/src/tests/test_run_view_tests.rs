use std::sync::mpsc::channel;
use egui_kittest::Harness;
use crate::views::{TestRunView, View};
use passivate_core::test_run_model::{BuildFailedTestRun, SingleTest, SingleTestStatus, TestRun, TestRunState};
use stdext::function_name;

#[test]
pub fn show_when_first_test_run_is_starting() {
    run_and_snapshot(TestRun::from_state(TestRunState::FirstRun), &test_name(function_name!()));
}

#[test]
pub fn show_when_no_tests_were_found() {
    run_and_snapshot(TestRun::from_state(TestRunState::Idle), &test_name(function_name!()));
}

#[test]
pub fn show_when_build_failed() {
    let build_failed = TestRun::from_state(TestRunState::BuildFailed(BuildFailedTestRun { 
        message: "Something didn't compile!".to_string() 
    }));

    run_and_snapshot(build_failed, &test_name(function_name!()));
}

#[test]
pub fn show_tests_with_unknown_status_greyed_out() {
    let mut active = TestRun::default();
    active.tests.push(SingleTest { name: "example_test".to_string(), status: SingleTestStatus::Unknown });

    run_and_snapshot(active, &test_name(function_name!()));
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