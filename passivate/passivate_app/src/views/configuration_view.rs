use passivate_core::{change_events::ChangeEvent, configuration::ConfigurationManager};
use passivate_delegation::Tx;

use crate::views::View;

pub struct ConfigurationView
{
    configuration_manager: ConfigurationManager,
    snapshots_path_field: String,
    change_event_tx: Tx<ChangeEvent>
}

impl ConfigurationView
{
    pub fn new(configuration_manager: ConfigurationManager, change_event_tx: Tx<ChangeEvent>) -> Self
    {
        Self {
            configuration_manager,
            snapshots_path_field: String::new(),
            change_event_tx
        }
    }
}

impl View for ConfigurationView
{
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        let mut configuration = self.configuration_manager.get_copy();

        if ui.toggle_value(&mut configuration.coverage_enabled, "Compute Coverage").changed()
        {
            self.configuration_manager.update(|c| {
                c.coverage_enabled = configuration.coverage_enabled;
            });

            self.change_event_tx.send(ChangeEvent::DefaultRun);
        }

        if let Some(configured_snapshots_path) = &configuration.snapshots_path
        {
            self.snapshots_path_field.clone_from(configured_snapshots_path);
        }

        ui.horizontal(|ui| {
            ui.label("Snapshots Path:");

            if ui.text_edit_singleline(&mut self.snapshots_path_field).lost_focus()
            {
                self.configuration_manager.update(|c| {
                    c.snapshots_path = Some(self.snapshots_path_field.clone());
                });

                self.change_event_tx.send(ChangeEvent::DefaultRun);
            }
        });
    }

    fn title(&self) -> String
    {
        "Configuration".to_string()
    }
}
