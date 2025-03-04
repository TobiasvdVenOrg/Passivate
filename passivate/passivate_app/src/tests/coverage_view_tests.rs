use std::sync::mpsc::channel;

use egui_kittest::Harness;
use crate::views::{CoverageView, View};
use passivate_core::{change_events::{ChangeEvent, HandleChangeEvent}, coverage::{CoverageError, MockComputeCoverage}, test_execution::{MockRunTests, TestRunner}};

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

    let mut coverage_view = CoverageView::new(coverage_receiver);

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    harness.run();

    harness.snapshot("when_grcov_is_not_installed_error_is_reported");
}