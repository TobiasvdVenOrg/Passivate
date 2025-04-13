use std::path::PathBuf;

use egui::accesskit::Role;
use egui_kittest::kittest::Key;
use egui_kittest::{Harness, kittest::Queryable};
use galvanic_assert::matchers::eq;
use galvanic_assert::{assert_that, has_structure, structure};
use passivate_core::delegation::{channel, Actor, Rx, Tx};
use passivate_core::configuration::{ConfigurationEvent, ConfigurationHandler, PassivateConfig};
use passivate_core::test_helpers::fakes::test_run_handler_fakes;
use passivate_core::test_run_model::Snapshots;
use stdext::function_name;

use crate::views::{ConfigurationView, DetailsView, View};


#[test]
pub fn show_configuration() {
    let (configuration_sender, configuration_receiver) = channel();

    let mut configuration_view = ConfigurationView::new(Tx::stub(), configuration_receiver, PassivateConfig::default());

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let new_configuration = PassivateConfig { 
        coverage_enabled: true, 
        snapshots_path: Some(String::from("tests/snapshots")) 
    };

    configuration_sender.send(ConfigurationEvent {
        old: None,
        new: new_configuration
    });

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn configure_coverage_enabled() {
    let change_handler = test_run_handler_fakes::stub();
    let (change_tx, mut change_actor) = Actor::new(change_handler);

    let configuration = ConfigurationHandler::new(Tx::from_actor(change_tx), Tx::stub());
    let (configuration_tx, mut configuration_actor) = Actor::new(configuration);

    let initial_configuration = PassivateConfig { coverage_enabled: false, ..PassivateConfig::default() };

    let mut configuration_view = ConfigurationView::new(Tx::from_actor(configuration_tx), Rx::stub(), initial_configuration);

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let coverage_toggle = harness.get_by_label("Compute Coverage");
    coverage_toggle.click();

    harness.run();

    let configuration = configuration_actor.into_inner();
    let change_handler = change_actor.into_inner();

    assert!(change_handler.coverage_enabled());
    assert!(configuration.configuration().coverage_enabled);
}

#[test]
pub fn configure_snapshots_path() {
    let initial_configuration = PassivateConfig::default();

    let (configuration_sender, configuration_receiver) = channel();

    let configuration = ConfigurationHandler::new(Tx::stub(), configuration_sender);
    let (configuration_tx, configuration_actor) = Actor::new(configuration);

    let mut configuration_view = ConfigurationView::new(Tx::from_actor(configuration_tx), Rx::stub(), initial_configuration);
    let mut details_view = DetailsView::new(Rx::stub(), Tx::stub(), configuration_receiver);

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
        details_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    harness.get_by_role(Role::TextInput).type_text("Some/Path/");
    harness.run();

    // Simulate typing across multiple frames...
    harness.get_by_role(Role::TextInput).type_text("To/Snapshots");
    harness.get_by_role(Role::TextInput).press_keys(&[ Key::Enter ]);
    harness.run();

    drop(harness);

    assert_that!(&details_view.get_snapshots(), structure!(Option<Snapshots>::Some [
        has_structure!(Snapshots {
            snapshot_directory: eq(PathBuf::from("Some/Path/To/Snapshots"))
        })
    ]));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}