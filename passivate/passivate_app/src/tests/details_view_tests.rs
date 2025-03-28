use std::sync::mpsc::channel;
use egui_kittest::{Harness, kittest::Queryable};
use crate::views::{DetailsView, TestRunView, View};
use passivate_core::test_run_model::{SingleTest, SingleTestStatus, TestRun};
use stdext::function_name;

#[test]
pub fn show_a_passing_test() {
    let failing_test = SingleTest::new("ExampleTest".to_string(), SingleTestStatus::Passed);
    
    show_test(&test_name(function_name!()), failing_test);
}

#[test]
pub fn show_a_failing_test() {
    let failing_test = SingleTest::new("ExampleTest".to_string(), SingleTestStatus::Failed);
    
    show_test(&test_name(function_name!()), failing_test);
}

#[test]
pub fn selecting_a_test_shows_it_in_details_view() {
    let (test_run_sender, test_run_receiver)  = channel();
    let (details_sender, details_receiver)  = channel();

    let mut details_view = DetailsView::new(details_receiver);
    let mut test_run_view = TestRunView::new(test_run_receiver, details_sender);

    let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui|{
        test_run_view.ui(ui);
    });

    let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui|{
        details_view.ui(ui);
    });

    let mut test_run = TestRun::default();
    test_run.tests.push(SingleTest::new("example_test".to_string(), SingleTestStatus::Failed));
    test_run_sender.send(test_run).unwrap();

    test_run_ui.run();

    let test_entry = test_run_ui.get_by_label("example_test");
    test_entry.click();

    test_run_ui.run();
    details_ui.run();

    details_ui.fit_contents();
    details_ui.snapshot(&test_name(function_name!()));
}

#[test]
pub fn show_snapshot_if_one_exists_matching_the_test_name() {
    let failing_test = SingleTest::new("ExampleTest".to_string(), SingleTestStatus::Passed);
    
    show_test(&test_name(function_name!()), failing_test);
}

fn show_test(test_name: &str, single_test: SingleTest) {
    let (sender, receiver)  = channel();
    let mut details_view = DetailsView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        details_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    sender.send(single_test).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(test_name);
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}