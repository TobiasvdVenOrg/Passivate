use std::sync::mpsc::channel;

use egui_kittest::{Harness, kittest::Queryable};
use passivate_core::configuration::{ConfigurationEvent, PassivateConfig};
use stdext::function_name;

use crate::views::{ConfigurationView, View};


#[test]
pub fn show_configuration() {
    let (configuration_change_sender, _configuration_change_receiver)  = channel();
    let (configuration_sender, configuration_receiver) = channel();

    let mut configuration_view = ConfigurationView::new(configuration_change_sender, configuration_receiver, PassivateConfig::default());

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let new_configuration = PassivateConfig { coverage_enabled: true };
    configuration_sender.send(new_configuration).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn change_coverage_enabled() {
    let (configuration_change_sender, configuration_change_receiver)  = channel();
    let (_configuration_sender, configuration_receiver) = channel();

    let mut configuration_view = ConfigurationView::new(configuration_change_sender, configuration_receiver, PassivateConfig::default());

    let ui = |ui: &mut egui::Ui|{
        configuration_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let toggle = harness.get_by_label("Compute Coverage");
    toggle.click();

    harness.run();

    let change_event = configuration_change_receiver.try_recv().unwrap();

    assert!(matches!(change_event, ConfigurationEvent::Coverage(true)));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}