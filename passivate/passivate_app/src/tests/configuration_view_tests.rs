use std::path::PathBuf;
use std::sync::mpsc::channel;

use egui_kittest::{Harness, kittest::Queryable};
use passivate_core::actors::Actor;
use passivate_core::configuration::{ConfigurationHandler, PassivateConfig};
use passivate_core::test_helpers::fakes::actor_fakes::stub_actor_api;
use passivate_core::test_helpers::fakes::channel_fakes::stub_sender;
use passivate_core::test_helpers::fakes::{channel_fakes, test_run_handler_fakes};
use stdext::function_name;

use crate::views::{ConfigurationView, View};


#[test]
pub fn show_configuration() {
    let (configuration_sender, configuration_receiver) = channel();

    let mut configuration_view = ConfigurationView::new(stub_actor_api(), configuration_receiver, PassivateConfig::default());

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let new_configuration = PassivateConfig { 
        coverage_enabled: true, 
        snapshots_path: Some(String::from("tests/snapshots")) 
    };

    configuration_sender.send(new_configuration).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn configure_coverage_enabled() {
    let change_handler = test_run_handler_fakes::stub();
    let mut change_actor = Actor::new(change_handler);

    let configuration = ConfigurationHandler::new(change_actor.api(), stub_sender());
    let mut configuration_actor = Actor::new(configuration);

    let configuration_receiver = channel_fakes::stub_receiver();
    
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

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}