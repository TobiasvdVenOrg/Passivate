use std::sync::mpsc::channel;

use egui_kittest::{Harness, kittest::Queryable};
use crate::views::{CoverageView, View};
use passivate_core::{change_events::{ChangeEvent, HandleChangeEvent}, coverage::{CoverageError, MockComputeCoverage}, test_execution::{MockRunTests, TestRunner}};

#[test]
pub fn enable_button_when_coverage_is_disabled_triggers_configuration_event() {
    let (_coverage_sender, coverage_receiver) = channel();
    let (change_event_sender, change_event_receiver) = channel();

    let mut coverage_view = CoverageView::new(coverage_receiver, change_event_sender);

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let enable_button = harness.get_by_label("Enable");
    enable_button.click();

    harness.run();

    let event = change_event_receiver.recv().unwrap();

    let configuration = event.as_configuration().unwrap();
    assert!(configuration.coverage_enabled);

    harness.snapshot("enable_button_when_coverage_is_disabled_triggers_configuration_event");
}

#[test]
pub fn when_grcov_is_not_installed_error_is_reported() {
    let mut run_tests = MockRunTests::new();
    run_tests.expect_run_tests().returning(|_sender| Ok(()));

    let mut compute_coverage = MockComputeCoverage::new();
    compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
    compute_coverage.expect_compute_coverage().returning(|| {
        Err(CoverageError::GrcovNotInstalled(std::io::ErrorKind::NotFound))
    });

    let (tests_sender, _tests_receiver) = channel();
    let (coverage_sender, coverage_receiver) = channel();
    let mut test_runner = TestRunner::new(Box::new(run_tests), Box::new(compute_coverage), tests_sender, coverage_sender);

    test_runner.handle_event(ChangeEvent::File);

    let (change_event_sender, _change_event_receiver) = channel();
    let mut coverage_view = CoverageView::new(coverage_receiver, change_event_sender);

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    harness.run();

    harness.snapshot("when_grcov_is_not_installed_error_is_reported");
}