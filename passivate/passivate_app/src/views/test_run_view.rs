use egui::{Color32, RichText};
use passivate_core::test_run_model::{SingleTest, SingleTestStatus, TestRun, TestRunState};
use std::sync::mpsc::{Receiver, Sender};
use crate::views::View;

pub struct TestRunView {
    receiver: Receiver<TestRun>,
    test_details: Sender<SingleTest>,
    status: TestRun
}

impl TestRunView {
    pub fn new(receiver: Receiver<TestRun>, test_details: Sender<SingleTest>) -> TestRunView {
        TestRunView { receiver, test_details, status: TestRun::default() }
    }

    fn test_button(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest, color: Color32) {
        let text = RichText::new(&test.name).size(16.0).color(color);
        
        if ui.button(text).clicked() {
            self.test_details.send(test.clone()).unwrap();
        }
    }

    fn test_label(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest) {
        let text = RichText::new(&test.name).size(16.0).color(Color32::GRAY);
        
        ui.label(text);
    }
}

impl View for TestRunView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match self.status.state {
            TestRunState::FirstRun => {
                ui.heading("Starting first test run...");
            }
            TestRunState::Idle => {
                        if self.status.tests.is_empty() {
                            ui.heading("No tests found."); 
                        }
                    },
            TestRunState::Building => todo!(),
            TestRunState::Running => {
                        
                    },
            TestRunState::BuildFailed(ref build_failure) => {
                        ui.heading("Build failed.");

                        let text = RichText::new(&build_failure.message).size(16.0).color(Color32::RED);
                        ui.label(text);
                    },
            TestRunState::Failed(ref run_tests_error_status) => {
                        ui.heading("Failed to run tests.");

                        let text = RichText::new(&run_tests_error_status.inner_error_display).size(16.0).color(Color32::RED);
                        ui.label(text);
                    }
        }

        for test in &self.status.tests {
            match test.status {
                SingleTestStatus::Failed => self.test_button(ui, test, Color32::RED),
                SingleTestStatus::Passed => self.test_button(ui, test, Color32::GREEN),
                SingleTestStatus::Unknown => self.test_label(ui, test)
            };
        }
    }

    fn title(&self) -> String {
        "Tests Status".to_string()
    }
}