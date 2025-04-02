use std::path::PathBuf;

use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::Queryable};
use galvanic_assert::matchers::eq;
use galvanic_assert::{assert_that, has_structure, structure};
use passivate_core::actors::Actor;
use passivate_core::configuration::{ConfigurationEvent, ConfigurationHandler, PassivateConfig};
use passivate_core::test_helpers::fakes::actor_fakes::stub_actor_api;
use passivate_core::test_helpers::fakes::channel_fakes::{stub_crossbeam_receiver, stub_crossbeam_sender, stub_receiver};
use passivate_core::test_helpers::fakes::{channel_fakes, test_run_handler_fakes};
use passivate_core::test_run_model::Snapshots;
use stdext::function_name;

use crate::views::{ConfigurationView, DetailsView, View};


#[test]
pub fn show_configuration() {
    let (configuration_sender, configuration_receiver) = crossbeam_channel::unbounded();

    let mut configuration_view = ConfigurationView::new(stub_actor_api(), configuration_receiver, PassivateConfig::default());

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
    }).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn configure_coverage_enabled() {
    let change_handler = test_run_handler_fakes::stub();
    let mut change_actor = Actor::new(change_handler);

    let configuration = ConfigurationHandler::new(change_actor.api(), stub_crossbeam_sender());
    let mut configuration_actor = Actor::new(configuration);

    let configuration_receiver = channel_fakes::stub_crossbeam_receiver();
    
    let initial_configuration = PassivateConfig { coverage_enabled: false, ..PassivateConfig::default() };

    let mut configuration_view = ConfigurationView::new(configuration_actor.api(), configuration_receiver, initial_configuration);

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let coverage_toggle = harness.get_by_label("Compute Coverage");
    coverage_toggle.click();

    harness.run();

    let configuration = configuration_actor.stop();
    let change_handler = change_actor.stop();

    assert!(change_handler.coverage_enabled());
    assert!(configuration.configuration().coverage_enabled);
}

#[test]
pub fn configure_snapshots_path() {
    let initial_configuration = PassivateConfig::default();

    let (configuration_sender, configuration_receiver) = crossbeam_channel::unbounded();

    let configuration = ConfigurationHandler::new(stub_actor_api(), configuration_sender);
    let configuration_actor = Actor::new(configuration);

    let mut configuration_view = ConfigurationView::new(configuration_actor.api(), stub_crossbeam_receiver(), initial_configuration);
    let mut details_view = DetailsView::new(stub_receiver(), stub_actor_api(), configuration_receiver);

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
        details_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let coverage_toggle = harness.get_by_role(Role::TextInput);
    coverage_toggle.type_text("Some/Path/To/Snapshots");

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