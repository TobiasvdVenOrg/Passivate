use egui::{Color32, RichText};
use passivate_core::{delegation::{Rx, Tx}, test_run_model::{SingleTest, SingleTestStatus, TestId, TestRun, TestRunState}};
use crate::views::View;

pub struct TestRunView {
    receiver: Rx<TestRun>,
    test_details: Tx<Option<SingleTest>>,
    status: TestRun,
    selected_test: Option<TestId>
}

impl TestRunView {
    pub fn new(receiver: Rx<TestRun>, test_details: Tx<Option<SingleTest>>) -> TestRunView {
        TestRunView { receiver, test_details, status: TestRun::default(), selected_test: None }
    }

    fn test_button(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest, color: Color32) -> Option<SingleTest> {
        let text = RichText::new(&test.name).size(16.0).color(color);
        
        if ui.button(text).clicked() {
            return Some(test.clone());
        }

        None
    }

    fn test_label(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest) {
        let text = RichText::new(&test.name).size(16.0).color(Color32::GRAY);
        
        ui.label(text);
    }

    fn send_selected_test_details(&self) {
        if let Some(selected_test) = &self.selected_test {
            self.test_details.send(self.status.tests.find(selected_test));
        }
    }

    fn show_test(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest) -> Option<SingleTest> {
        match test.status {
            SingleTestStatus::Failed => self.test_button(ui, test, Color32::RED),
            SingleTestStatus::Passed => self.test_button(ui, test, Color32::GREEN),
            SingleTestStatus::Unknown => { self.test_label(ui, test); None }
        }
    }
}

impl View for TestRunView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
            self.send_selected_test_details();
        }

        match &self.status.state {
            TestRunState::FirstRun => {
                ui.heading("Starting first test run...");
            }
            TestRunState::Idle => {
                        if self.status.tests.is_empty() {
                            ui.heading("No tests found."); 
                        }
                    },
            TestRunState::Building(message) => {
                ui.heading("Building");

                let text = RichText::new(message).size(12.0).color(Color32::YELLOW);
                ui.label(text);
            },
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
            if let Some(new_selection) = self.show_test(ui, test) {
                self.selected_test = Some(new_selection.id());
                self.send_selected_test_details();
            }
        }
    }

    fn title(&self) -> String {
        "Tests Status".to_string()
    }
}