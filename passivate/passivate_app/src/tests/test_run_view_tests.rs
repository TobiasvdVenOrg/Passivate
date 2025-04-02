use std::sync::mpsc::channel;
use egui_kittest::Harness;
use crate::views::{TestRunView, View};
use passivate_core::{test_helpers::fakes::channel_fakes::stub_sender, test_run_model::{BuildFailedTestRun, SingleTest, SingleTestStatus, TestRun, TestRunEvent, TestRunState}};
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
    active.tests.add(example_test("example_test", SingleTestStatus::Unknown));

    run_and_snapshot(active, &test_name(function_name!()));
}

#[test]
pub fn show_build_status_above_tests_while_compiling() {
    let mut active = TestRun::default();
    active.tests.add(example_test("example_test", SingleTestStatus::Unknown));
    active.update(TestRunEvent::Compiling("The build is working on something right now!".to_string()));

    run_and_snapshot(active, &test_name(function_name!()));
}

fn run_and_snapshot(tests_status: TestRun, snapshot_name: &str) {
    let (sender, receiver)  = channel();
    let mut tests_status_view = TestRunView::new(receiver, stub_sender());

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

fn example_test(name: &str, status: SingleTestStatus) -> SingleTest {
    SingleTest::new(name.to_string(), status, vec![])
}
