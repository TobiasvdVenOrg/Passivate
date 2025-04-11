use passivate_core::{configuration::{ConfigurationChangeEvent, ConfigurationEvent, PassivateConfig}, delegation::Give};

use crate::views::View;

pub struct ConfigurationView {
    stakeholder: Box<dyn Give<ConfigurationChangeEvent>>,
    receiver: crossbeam_channel::Receiver<ConfigurationEvent>,
    configuration: PassivateConfig,

    snapshots_path_field: String
}

impl ConfigurationView {
    pub fn new(stakeholder: Box<dyn Give<ConfigurationChangeEvent>>, receiver: crossbeam_channel::Receiver<ConfigurationEvent>, configuration: PassivateConfig) -> Self {
        Self { stakeholder, receiver, configuration, snapshots_path_field: String::new() }
    }
}

impl View for ConfigurationView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_configuration) = self.receiver.try_recv() {
            self.configuration = new_configuration.new;
        }

        if ui.toggle_value(&mut self.configuration.coverage_enabled, "Compute Coverage").changed() {
            self.stakeholder.send(ConfigurationChangeEvent::Coverage(self.configuration.coverage_enabled));
        }

        if let Some(configured_snapshots_path) = &self.configuration.snapshots_path {
            self.snapshots_path_field.clone_from(configured_snapshots_path);
        }

        ui.horizontal(|ui| {
            ui.label("Snapshots Path:");

            if ui.text_edit_singleline(&mut self.snapshots_path_field).lost_focus() {
                self.stakeholder.send(ConfigurationChangeEvent::SnapshotsPath(self.snapshots_path_field.clone()));
            }
        });
    }

    fn title(&self) -> String {
        "Configuration".to_string()
    }
}
