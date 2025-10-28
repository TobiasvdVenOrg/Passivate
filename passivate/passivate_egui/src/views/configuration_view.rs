use camino::Utf8PathBuf;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_delegation::Tx;
use passivate_hyp_model::hyp_run_trigger::HypRunTrigger;

pub struct ConfigurationView
{
    configuration_manager: ConfigurationManager,
    snapshots_path_field: String,
    change_event_tx: Tx<HypRunTrigger>
}

impl ConfigurationView
{
    pub fn new(configuration_manager: ConfigurationManager, change_event_tx: Tx<HypRunTrigger>) -> Self
    {
        Self {
            configuration_manager,
            snapshots_path_field: String::new(),
            change_event_tx
        }
    }

    pub fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        let mut configuration = self.configuration_manager.get_copy();

        if ui
            .toggle_value(&mut configuration.coverage_enabled, "Compute Coverage")
            .changed()
        {
            _ = self.configuration_manager.update(|c| {
                c.coverage_enabled = configuration.coverage_enabled;
            });

            self.change_event_tx.send(HypRunTrigger::DefaultRun);
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
            _ = self.configuration_manager.update(|c| {
                c.snapshot_directories.push(Utf8PathBuf::from(self.snapshots_path_field.as_str()));
            });

            self.snapshots_path_field = String::new();
            self.change_event_tx.send(HypRunTrigger::DefaultRun);
        }
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
    use passivate_delegation::Tx;
    use passivate_hyp_execution::test_run_handler::TestRunHandler;
    use passivate_hyp_execution::hyp_runner::HypRunner;
    use passivate_hyp_model::hyp_run_trigger::HypRunTrigger;
    use passivate_hyp_names::test_name;

    use crate::configuration_view::ConfigurationView;

    #[test]
    pub fn show_configuration()
    {
        let mut configuration_manager = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());
        let mut configuration_view = ConfigurationView::new(configuration_manager.clone(), Tx::stub());

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        configuration_manager
            .update(|c| {
                c.coverage_enabled = true;
                c.snapshot_directories.push(Utf8PathBuf::from("tests/snapshots"));
            })
            .unwrap();

        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn configure_coverage_enabled()
    {
        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());
        let test_run_handler = TestRunHandler::builder()
            .configuration(configuration.clone())
            .coverage(Box::new(MockComputeCoverage::new()))
            .coverage_status_sender(Tx::stub())
            .runner(HypRunner::faux())
            .hyp_run_tx(Tx::stub())
            .build();

        let (change_events_tx, change_events_rx) = Tx::new();
        let mut configuration_view = ConfigurationView::new(configuration, change_events_tx);

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        let coverage_toggle = harness.get_by_label("Compute Coverage");
        coverage_toggle.click();

        harness.run();

        assert_that!(
            &change_events_rx.drain().last().expect("expected change event").clone(),
            eq(HypRunTrigger::DefaultRun)
        );

        assert!(test_run_handler.coverage_enabled());
    }

    #[test]
    pub fn configuring_snapshots_path_starts_test_run()
    {
        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());
        let (change_events_tx, change_events_rx) = Tx::new();
        let mut configuration_view = ConfigurationView::new(configuration.clone(), change_events_tx);

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        harness.get_by_role(Role::TextInput).type_text("Some/Path/");
        harness.run();

        // Simulate typing across multiple frames...
        harness.get_by_role(Role::TextInput).type_text("To/Snapshots");
        harness.get_by_role(Role::TextInput).press_keys(&[Key::Enter]);
        harness.run();

        drop(harness);

        assert_that!(
            &change_events_rx.drain().last().expect("expected change event").clone(),
            eq(HypRunTrigger::DefaultRun)
        );

        assert_eq!(
            configuration.get(|c| c.snapshot_directories.iter().exactly_one().unwrap().clone()).as_str(),
            "Some/Path/To/Snapshots"
        );
    }
}
