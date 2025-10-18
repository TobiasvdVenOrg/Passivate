use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::change_events::ChangeEvent;
use passivate_delegation::Tx;

use crate::docking::{dock_state::DockId, view::View};

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

    fn id(&self) -> DockId
    {
        "configuration_view".into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        let mut configuration = self.configuration_manager.get_copy();

        if ui.toggle_value(&mut configuration.coverage_enabled, "Compute Coverage").changed()
        {
            _ = self.configuration_manager.update(|c| {
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
                _ = self.configuration_manager.update(|c| {
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

#[cfg(test)]
mod tests
{
    use camino::Utf8PathBuf;
    use egui::accesskit::Role;
    use egui_kittest::kittest::{Key, Queryable};
    use egui_kittest::Harness;
    use galvanic_assert::{has_structure, structure};
    use galvanic_assert::{assert_that, matchers::eq};
    use passivate_configuration::{configuration::Configuration, configuration_manager::ConfigurationManager};
    use passivate_core::test_run_model::Snapshots;
    use passivate_core::{change_events::ChangeEvent, coverage::MockComputeCoverage, test_execution::{TestRunHandler, TestRunner}};
    use passivate_delegation::{Rx, Tx};
    use passivate_hyp_names::test_name;

    use crate::details_view::DetailsView;
    use crate::configuration_view::ConfigurationView;
    use crate::docking::view::View;

    #[test]
    pub fn show_configuration()
    {
        let mut configuration_manager = ConfigurationManager::new(Configuration::default(), Tx::stub());
        let mut configuration_view = ConfigurationView::new(configuration_manager.clone(), Tx::stub());

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        configuration_manager.update(|c| {
            c.coverage_enabled = true;
            c.snapshots_path = Some(String::from("tests/snapshots"));
        }).unwrap();

        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn configure_coverage_enabled()
    {
        let configuration = ConfigurationManager::new(Configuration::default(), Tx::stub());
        let test_run_handler = TestRunHandler::builder()
            .configuration(configuration.clone())
            .coverage(Box::new(MockComputeCoverage::new()))
            .coverage_status_sender(Tx::stub())
            .runner(TestRunner::faux())
            .tests_status_sender(Tx::stub())
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

        assert_that!(&change_events_rx.last().expect("expected change event"), eq(ChangeEvent::DefaultRun));

        assert!(test_run_handler.coverage_enabled());
    }

    #[test]
    pub fn configure_snapshots_path()
    {
        let configuration = ConfigurationManager::new(Configuration::default(), Tx::stub());
        let (change_events_tx, change_events_rx) = Tx::new();
        let mut configuration_view = ConfigurationView::new(configuration.clone(), change_events_tx);
        let mut details_view = DetailsView::new(Rx::stub(), Tx::stub(), configuration);

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui);
            details_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        harness.get_by_role(Role::TextInput).type_text("Some/Path/");
        harness.run();

        // Simulate typing across multiple frames...
        harness.get_by_role(Role::TextInput).type_text("To/Snapshots");
        harness.get_by_role(Role::TextInput).press_keys(&[Key::Enter]);
        harness.run();

        drop(harness);

        assert_that!(&change_events_rx.last().expect("expected change event"), eq(ChangeEvent::DefaultRun));

        assert_that!(
            &details_view.get_snapshots(),
            structure!(Option<Snapshots>::Some [
                has_structure!(Snapshots {
                    snapshot_directory: eq(Utf8PathBuf::from("Some/Path/To/Snapshots"))
                })
            ])
        );
    }
}
