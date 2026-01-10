use camino::Utf8PathBuf;
use egui::Ui;
use passivate_configuration::configuration::PassivateConfiguration;

pub struct ConfigurationView
{
    snapshots_path_field: String
}

pub enum ConfigurationViewAction
{
    RequestRerun,
    UpdateConfiguration(Box<dyn FnOnce(&mut PassivateConfiguration)>)
}

impl ConfigurationView
{
    pub fn new() -> Self
    {
        Self {
            snapshots_path_field: String::new()
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, configuration: &PassivateConfiguration) -> Vec<ConfigurationViewAction>
    {
        let mut actions = Vec::new();
        let mut coverage_enabled = configuration.coverage_enabled;

        if ui.toggle_value(&mut coverage_enabled, "Compute Coverage").changed()
        {
            actions.push(ConfigurationViewAction::UpdateConfiguration(Box::new(move |c| {
                c.coverage_enabled = coverage_enabled;
            })));

            actions.push(ConfigurationViewAction::RequestRerun);
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

            actions.push(ConfigurationViewAction::UpdateConfiguration(Box::new(|c| {
                c.snapshot_directories.push(new_snapshots_path);
            })));

            actions.push(ConfigurationViewAction::RequestRerun);

            self.snapshots_path_field = String::new();
        }

        actions
    }
}

#[cfg(test)]
mod tests
{
    use camino::Utf8PathBuf;
    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{Key, Queryable};
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::eq;
    use itertools::Itertools;
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_coverage::compute_coverage::MockComputeCoverage;
    use passivate_delegation::{MockTx, Tx};
    use passivate_hyp_names::test_name;
    use passivate_model_rust::RustBridge;
    use passivate_run_rust::hyp_runner::HypRunner;

    use crate::ConfigurationView;

    #[test]
    pub fn show_configuration()
    {
        let mut configuration_view = ConfigurationView::new();

        let configuration = PassivateConfiguration {
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
