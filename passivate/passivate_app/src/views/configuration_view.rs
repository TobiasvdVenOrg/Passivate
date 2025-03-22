use std::sync::mpsc::Receiver;
use passivate_core::{actors::ActorApi, configuration::{ConfigurationChangeEvent, PassivateConfig}};

use crate::views::View;

pub struct ConfigurationView {
    sender: ActorApi<ConfigurationChangeEvent>,
    receiver: Receiver<PassivateConfig>,
    configuration: PassivateConfig
}

impl ConfigurationView {
    pub fn new(sender: ActorApi<ConfigurationChangeEvent>, receiver: Receiver<PassivateConfig>, configuration: PassivateConfig) -> Self {
        Self { sender, receiver, configuration }
    }
}

impl View for ConfigurationView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_configuration) = self.receiver.try_recv() {
            self.configuration = new_configuration;
        }

        if ui.toggle_value(&mut self.configuration.coverage_enabled, "Compute Coverage").changed() {
            self.sender.send(ConfigurationChangeEvent::Coverage(self.configuration.coverage_enabled));
        }
    }

    fn title(&self) -> String {
        "Configuration".to_string()
    }
}
