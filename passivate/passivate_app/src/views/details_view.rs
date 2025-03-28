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
            let color = match single_test.status {
                passivate_core::test_run_model::SingleTestStatus::Passed => Color32::GREEN,
                passivate_core::test_run_model::SingleTestStatus::Failed => Color32::RED,
                passivate_core::test_run_model::SingleTestStatus::Unknown => Color32::GRAY,
            };

            let text = RichText::new(&single_test.name).size(16.0).color(color);
            ui.heading(text);
        }
    }

    fn title(&self) -> String {
        "Details".to_string()
    }
}
