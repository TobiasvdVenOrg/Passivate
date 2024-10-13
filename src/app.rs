use std::sync::{Arc, RwLock};
use eframe::{Frame};
use egui::Context;
use passivate_core::passivate_notify::NotifyChangeEvents;
use passivate_core::tests_view::TestsStatus;

pub struct App {
    status: Arc<RwLock<TestsStatus>>,
    change_events: Box<NotifyChangeEvents>
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.status.read().unwrap().text.as_str());
        });
    }
}

impl App {
    pub fn new(status: Arc<RwLock<TestsStatus>>, change_events: Box<NotifyChangeEvents>) -> Self {
        App { status, change_events }
    }
}