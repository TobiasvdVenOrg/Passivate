use std::process::Command;
use std::sync::{Arc, RwLock};
use eframe::{Frame};
use egui::{Color32, Context, RichText};
use passivate_core::passivate_notify::NotifyChangeEvents;
use passivate_core::tests_view::{SingleTestStatus, TestsStatus};

pub struct App {
    status: Arc<RwLock<TestsStatus>>,
    change_events: NotifyChangeEvents
}

impl App {
    pub fn new(status: Arc<RwLock<TestsStatus>>, change_events: NotifyChangeEvents) -> Self {
        App { status, change_events }
    }

    pub fn boxed(status: Arc<RwLock<TestsStatus>>, change_events: NotifyChangeEvents) -> Box<App> {
        Box::new(Self::new(status, change_events))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            if let Some(status) = self.status.read().ok() {
                if status.running {
                    ui.heading("Running tests...");
                }
                else {
                    for test in &status.tests {
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

                    if status.tests.is_empty() {
                        ui.heading("Make a change to discover tests!");
                    }
                }
            }
        });
    }
}
