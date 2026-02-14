use camino::Utf8PathBuf;
use egui::Ui;
use passivate_configuration::configuration::{ConfigurationChange, PassivateConfiguration};

#[derive(Default)]
pub struct ConfigurationView
{
    snapshots_path_field: String
}

impl ConfigurationView
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut Ui, configuration: &PassivateConfiguration) -> Vec<ConfigurationChange>
    {
        let mut actions = Vec::new();
        let mut coverage_enabled = configuration.coverage_enabled;

        if ui.toggle_value(&mut coverage_enabled, "Compute Coverage").changed()
        {
            actions.push(ConfigurationChange::CoverageEnabled(coverage_enabled));
        }

        ui.label("Snapshot Directories");

        for snapshot_directory in &configuration.snapshot_directories
        {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(snapshot_directory.as_str());
            });
        }

        if ui.text_edit_singleline(&mut self.snapshots_path_field).lost_focus()
        {
            let new_snapshots_path = Utf8PathBuf::from(&self.snapshots_path_field);

            actions.push(ConfigurationChange::AddSnapshotDirectory(new_snapshots_path));

            self.snapshots_path_field = String::new();
        }

        actions
    }
}

#[cfg(test)]
mod tests
{
    use camino::Utf8PathBuf;
    use egui_kittest::Harness;
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_hyp_names::test_name;

    use crate::ConfigurationView;

    #[test]
    pub fn show_configuration()
    {
        let mut configuration_view = ConfigurationView::new();

        let configuration = PassivateConfiguration {
            passivate_directory: Some(Utf8PathBuf::from("some/alternative/.passivate")),
            coverage_enabled: true,
            snapshot_directories: vec![Utf8PathBuf::from("tests/snapshots")]
        };

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui, &configuration);
        };

        let mut harness = Harness::new_ui(ui);
        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }
}
