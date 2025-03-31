use std::sync::mpsc::channel;
use egui_kittest::{Harness, kittest::Queryable};
use crate::views::{DetailsView, TestRunView, View};
use passivate_core::test_run_model::{SingleTest, SingleTestStatus, Snapshots, TestRun, TestRunEvent};
use passivate_core::test_helpers::builder::test_data_path;
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
    test_run.tests.add(SingleTest::new("example_test".to_string(), SingleTestStatus::Failed));
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
pub fn when_a_test_is_selected_and_updates_the_details_view_also_updates() {
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
    test_run.update(TestRunEvent::TestFinished(SingleTest::new("example_test".to_string(), SingleTestStatus::Failed)));
    test_run_sender.send(test_run.clone()).unwrap();

    test_run_ui.run();

    let test_entry = test_run_ui.get_by_label("example_test");
    test_entry.click();

    test_run_ui.run();
    details_ui.run();

    test_run.update(TestRunEvent::TestFinished(SingleTest::new("example_test".to_string(), SingleTestStatus::Passed)));
    test_run_sender.send(test_run).unwrap();

    test_run_ui.run();
    details_ui.run();

    details_ui.fit_contents();
    details_ui.snapshot(&test_name(function_name!()));
}

#[test]
pub fn show_snapshot_associated_with_test_rgb() {
    let test_with_snapshot = SingleTest::new("example_snapshot_rgb".to_string(), SingleTestStatus::Failed);
    
    show_test(&test_name(function_name!()), test_with_snapshot);
}

#[test]
pub fn show_snapshot_associated_with_test_rgba() {
    let test_with_snapshot = SingleTest::new("example_snapshot_rgba".to_string(), SingleTestStatus::Failed);
    
    show_test(&test_name(function_name!()), test_with_snapshot);
}

#[test]
pub fn show_current_and_new_snapshots_associated_with_test() {
    let test_with_changed_snapshot = SingleTest::new("example_snapshot_changed".to_string(), SingleTestStatus::Failed);
    
    show_test(&test_name(function_name!()), test_with_changed_snapshot);
}

#[test]
pub fn show_only_new_snapshot_associated_with_test_when_there_is_no_current_snapshot() {
    let test_first_run = SingleTest::new("example_snapshot_only_new".to_string(), SingleTestStatus::Failed);
    
    show_test(&test_name(function_name!()), test_first_run);
}

fn show_test(test_name: &str, single_test: SingleTest) {
    let (sender, receiver)  = channel();

    let mut details_view = DetailsView::new(receiver);
    details_view.set_snapshots(Snapshots::new(test_data_path().join("example_snapshots")));

    let ui = |ui: &mut egui::Ui|{
        details_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    sender.send(Some(single_test)).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(test_name);
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}