use std::sync::mpsc::Receiver;
use eframe::{Frame};
use egui::{Color32, Context, RichText};
use crate::passivate_notify::{NotifyChangeEvents, NotifyChangeEventsError};
use passivate_core::test_execution::{SingleTestStatus, TestsStatus};

pub struct App {
    receiver: Receiver<TestsStatus>,
    change_events: NotifyChangeEvents,
    status: TestsStatus
}

impl App {
    pub fn new(receiver: Receiver<TestsStatus>, change_events: NotifyChangeEvents) -> Self {
        let status = TestsStatus::waiting();
        App { receiver, change_events, status }
    }

    pub fn boxed(status: Receiver<TestsStatus>, change_events: NotifyChangeEvents) -> Box<App> {
        Box::new(Self::new(status, change_events))
    }

    fn stop(&mut self) -> Result<(), NotifyChangeEventsError> {
        self.change_events.stop()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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

            ui.add_space(ui.available_height() - 16.0);
            if ui.button("Stop").clicked() {
                match self.stop() {
                    Ok(_) => {
                        println!("Stopped listening for change events.");
                    }
                    Err(_) => {
                        println!("Failed to stop listening for change events.");
                    }
                }
            }
        });

        ctx.request_repaint();
    }
}
