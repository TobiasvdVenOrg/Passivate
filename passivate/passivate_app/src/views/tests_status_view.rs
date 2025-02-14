use egui::{Color32, RichText};
use std::sync::mpsc::Receiver;
use passivate_core::test_execution::{SingleTestStatus, TestsStatus};
use crate::views::View;

pub struct TestsStatusView {
    receiver: Receiver<TestsStatus>,
    status: TestsStatus
}

impl TestsStatusView {
    pub fn new(receiver: Receiver<TestsStatus>) -> TestsStatusView {
        TestsStatusView { receiver, status: TestsStatus::waiting() }
    }
}

impl View for TestsStatusView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match self.status {
            TestsStatus::Waiting => {
                ui.heading("Make a change to discover tests!");
            },
            TestsStatus::Running => {
                ui.heading("Running tests...");
            },
            TestsStatus::Completed(ref completed) => {
                for test in &completed.tests {
                    let color = match test.status {
                        SingleTestStatus::Failed => Color32::RED,
                        SingleTestStatus::Passed => Color32::GREEN
                    };

                    let text = RichText::new(&test.name).size(16.0).color(color);
                    if ui.button(text).clicked() {
                        println!("Clicked on {}", test.name);
                        //Command::new("rustrover").arg("test")
                    }
                }

                if completed.tests.is_empty() {
                    ui.heading("No tests found.");
                }
            },
            TestsStatus::BuildFailure(ref build_failure) => {
                ui.heading("Build failed.");

                let text = RichText::new(&build_failure.message).size(16.0).color(Color32::RED);
                ui.label(text);
            }
        }
    }

    fn title(&self) -> String {
        "Tests Status".to_string()
    }
}