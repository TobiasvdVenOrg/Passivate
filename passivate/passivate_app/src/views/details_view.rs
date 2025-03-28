use std::sync::mpsc::Receiver;

use egui::{Color32, RichText};
use passivate_core::test_run_model::SingleTest;

use super::View;


pub struct DetailsView {
    receiver: Receiver<SingleTest>,
    single_test: Option<SingleTest>
}

impl DetailsView {
    pub fn new(receiver: Receiver<SingleTest>) -> Self {
        Self { receiver, single_test: None }
    }
}

impl View for DetailsView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_test) = self.receiver.try_recv() {
            self.single_test = Some(new_test);
        }

        if let Some(single_test) = &self.single_test {
            let text = RichText::new(&single_test.name).size(16.0).color(Color32::GREEN);
            ui.heading(text);
        }
    }

    fn title(&self) -> String {
        "Details".to_string()
    }
}
