use egui::{Color32, RichText};
use passivate_core::test_run_model::{SingleTestStatus, TestRun};
use std::sync::mpsc::Receiver;
use crate::views::View;

pub struct TestRunView {
    receiver: Receiver<TestRun>,
    status: TestRun
}

impl TestRunView {
    pub fn new(receiver: Receiver<TestRun>) -> TestRunView {
        TestRunView { receiver, status: TestRun::Waiting { } }
    }
}

impl View for TestRunView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match self.status {
            TestRun::Waiting => {
                ui.heading("Make a change to discover tests!");
            },
            TestRun::Starting => {
                ui.heading("Starting test run...");
            },
            TestRun::Active(ref active) => {
                for test in &active.tests {
                    let color = match test.status {
                        SingleTestStatus::Failed => Color32::RED,
                        SingleTestStatus::Passed => Color32::GREEN
                    };

                    let text = RichText::new(&test.name).size(16.0).color(color);
                    if ui.button(text).clicked() {
                        println!("Clicked on {}", test.name);
                    }
                }

                if active.tests.is_empty() {
                    ui.heading("No tests found.");
                }
            },
            TestRun::BuildFailed(ref build_failure) => {
                ui.heading("Build failed.");

                let text = RichText::new(&build_failure.message).size(16.0).color(Color32::RED);
                ui.label(text);
            },
            TestRun::Failed(ref run_tests_error_status) => {
                ui.heading("Failed to run tests.");

                let text = RichText::new(&run_tests_error_status.inner_error_display).size(16.0).color(Color32::RED);
                ui.label(text);
            },
        }
    }

    fn title(&self) -> String {
        "Tests Status".to_string()
    }
}