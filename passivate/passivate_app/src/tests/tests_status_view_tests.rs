use std::sync::mpsc::channel;

use egui_kittest::Harness;
use crate::views::{TestsStatusView, View};
use passivate_core::test_execution::TestsStatus;

#[test]
pub fn tests_status_view_shows_when_tests_are_running() {
    let (sender, receiver)  = channel();
    let mut tests_status_view = TestsStatusView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        tests_status_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    sender.send(TestsStatus::Running).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot("tests_status_view_shows_when_tests_are_running");
}