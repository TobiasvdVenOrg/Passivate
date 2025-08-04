use std::path::PathBuf;

use egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::{Key, Queryable};
use galvanic_assert::matchers::eq;
use galvanic_assert::{assert_that, has_structure, structure};
use passivate_core::configuration::{ConfigurationManager, PassivateConfig};
use passivate_core::test_helpers::fakes::test_run_actor_fakes;
use passivate_core::test_run_model::Snapshots;
use passivate_delegation::{Rx, Tx};
use stdext::function_name;

use crate::views::{ConfigurationView, DetailsView, View};

#[test]
pub fn show_configuration()
{
    let mut configuration_manager = ConfigurationManager::new(PassivateConfig::default(), Tx::stub(), Tx::stub());
    let mut configuration_view = ConfigurationView::new(configuration_manager.clone());

    let ui = |ui: &mut egui::Ui| {
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    configuration_manager.update(|c| {
        c.coverage_enabled = true;
        c.snapshots_path = Some(String::from("tests/snapshots"));
    });

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn configure_coverage_enabled()
{
    let configuration = ConfigurationManager::new(PassivateConfig::default(), Tx::stub(), Tx::stub());
    let test_run_handler = test_run_actor_fakes::stub_with_coverage_enabled(|| configuration.get(|c| c.coverage_enabled));

    let mut configuration_view = ConfigurationView::new(configuration.clone());

    let ui = |ui: &mut egui::Ui| {
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let coverage_toggle = harness.get_by_label("Compute Coverage");
    coverage_toggle.click();

    harness.run();

    assert!(test_run_handler.coverage_enabled());
}

#[test]
pub fn configure_snapshots_path()
{
    let configuration = ConfigurationManager::new(PassivateConfig::default(), Tx::stub(), Tx::stub());
    let mut configuration_view = ConfigurationView::new(configuration.clone());
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

    assert_that!(
        &details_view.get_snapshots(),
        structure!(Option<Snapshots>::Some [
            has_structure!(Snapshots {
                snapshot_directory: eq(PathBuf::from("Some/Path/To/Snapshots"))
            })
        ])
    );
}

fn test_name(function_name: &str) -> String
{
    function_name.split("::").last().unwrap_or(function_name).to_string()
}
