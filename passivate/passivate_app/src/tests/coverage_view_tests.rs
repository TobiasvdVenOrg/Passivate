use std::sync::mpsc::channel;
use egui_kittest::{Harness, kittest::Queryable};
use passivate_core::actors::{Actor, Handler};
use crate::views::{CoverageView, View};

struct SpyHandler<T: Send + 'static> {
    events: Vec<T>
}

impl<T: Send + 'static> Handler<T> for SpyHandler<T> {
    fn handle(&mut self, event: T) {
        self.events.push(event);
    }
}

impl<T: Send + 'static> Default for SpyHandler<T> {
    fn default() -> Self {
        Self { events: Default::default() }
    }
}

#[test]
pub fn enable_button_when_coverage_is_disabled_triggers_configuration_event() {
    let (_coverage_sender, coverage_receiver) = channel();
    let spy = SpyHandler::default();
    let mut actor = Actor::new(spy);

    let mut coverage_view = CoverageView::new(coverage_receiver, actor.api());

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let enable_button = harness.get_by_label("Enable");
    enable_button.click();

    harness.run();

    let spy = actor.stop();

    let event = spy.events.last().unwrap();

    let configuration = event.as_configuration().unwrap();
    assert!(configuration.coverage_enabled);

    harness.fit_contents();
    harness.snapshot("enable_button_when_coverage_is_disabled_triggers_configuration_event");
}

// #[test]
// pub fn when_grcov_is_not_installed_error_is_reported() {
//     let mut run_tests = MockRunTests::new();
//     run_tests.expect_run_tests().returning(|_sender| Ok(()));

//     let mut compute_coverage = MockComputeCoverage::new();
//     compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
//     compute_coverage.expect_compute_coverage().returning(|| {
//         Err(CoverageError::GrcovNotInstalled(std::io::ErrorKind::NotFound))
//     });

//     let (tests_sender, _tests_receiver) = channel();
//     let (coverage_sender, coverage_receiver) = channel();
//     let mut test_runner = ChangeEventHandler::new(Box::new(run_tests), Box::new(compute_coverage), tests_sender, coverage_sender);

//     test_runner.handle_event(ChangeEvent::File);

//     let (change_event_sender, _change_event_receiver) = channel();
//     let mut coverage_view = CoverageView::new(coverage_receiver, change_event_sender);

//     let ui = |ui: &mut egui::Ui|{
//         coverage_view.ui(ui);
//     };

//     let mut harness = Harness::new_ui(ui);

//     harness.run();

//     harness.fit_contents();
//     harness.snapshot("when_grcov_is_not_installed_error_is_reported");
// }
