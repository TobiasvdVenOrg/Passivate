use std::sync::mpsc::{Receiver, Sender};
use passivate_core::configuration::{ConfigurationEvent, PassivateConfig};

use crate::views::View;

pub struct ConfigurationView {
    sender: Sender<ConfigurationEvent>,
    receiver: Receiver<PassivateConfig>,
    configuration: PassivateConfig
}

impl ConfigurationView {
    pub fn new(sender: Sender<ConfigurationEvent>, receiver: Receiver<PassivateConfig>, configuration: PassivateConfig) -> Self {
        Self { sender, receiver, configuration }
    }
}

impl View for ConfigurationView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_configuration) = self.receiver.try_recv() {
            self.configuration = new_configuration;
        }

        ui.checkbox(&mut self.configuration.coverage_enabled, "Compute Coverage");
    }

    fn title(&self) -> String {
        "Configuration".to_string()
    }
}
