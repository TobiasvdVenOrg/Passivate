use passivate_core::{actors::ActorApi, configuration::{ConfigurationChangeEvent, ConfigurationEvent, PassivateConfig}};

use crate::views::View;

pub struct ConfigurationView {
    sender: ActorApi<ConfigurationChangeEvent>,
    receiver: crossbeam_channel::Receiver<ConfigurationEvent>,
    configuration: PassivateConfig
}

impl ConfigurationView {
    pub fn new(sender: ActorApi<ConfigurationChangeEvent>, receiver: crossbeam_channel::Receiver<ConfigurationEvent>, configuration: PassivateConfig) -> Self {
        Self { sender, receiver, configuration }
    }
}

impl View for ConfigurationView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_configuration) = self.receiver.try_recv() {
            self.configuration = new_configuration.new;
        }

        if ui.toggle_value(&mut self.configuration.coverage_enabled, "Compute Coverage").changed() {
            self.sender.send(ConfigurationChangeEvent::Coverage(self.configuration.coverage_enabled));
        }

        let mut snapshots_path = String::new();

        if let Some(configured_snapshots_path) = &self.configuration.snapshots_path {
            snapshots_path.clone_from(configured_snapshots_path);
        }

        ui.horizontal(|ui| {
            ui.label("Snapshots Path:");

            if ui.text_edit_singleline(&mut snapshots_path).changed() {
                self.sender.send(ConfigurationChangeEvent::SnapshotsPath(snapshots_path));
            }
        });
    }

    fn title(&self) -> String {
        "Configuration".to_string()
    }
}
