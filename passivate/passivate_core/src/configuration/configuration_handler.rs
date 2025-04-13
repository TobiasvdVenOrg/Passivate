use crate::{change_events::ChangeEvent, delegation::{Cancellation, Tx, Handler}};

use super::{ConfigurationChangeEvent, ConfigurationEvent, PassivateConfig};


pub struct ConfigurationHandler {
    configuration: PassivateConfig,
    change_handler: Tx<ChangeEvent>,
    configuration_handler: Tx<ConfigurationEvent>
}

impl ConfigurationHandler {
    pub fn new(change_handler: Tx<ChangeEvent>, configuration_handler: Tx<ConfigurationEvent>) -> Self {
        Self { configuration: PassivateConfig::default(), change_handler, configuration_handler }
    }

    pub fn configuration(&self) -> PassivateConfig {
        self.configuration.clone()
    }
}

impl Handler<ConfigurationChangeEvent> for ConfigurationHandler {
    fn handle(&mut self, event: ConfigurationChangeEvent, _cancellation: Cancellation) {
        let old = self.configuration.clone();

        match event {
            ConfigurationChangeEvent::Coverage(enabled) => {
                self.configuration.coverage_enabled = enabled;
            }
            ConfigurationChangeEvent::SnapshotsPath(snapshots_path) => {
                self.configuration.snapshots_path = Some(snapshots_path);
            }
        }

        let configuration_event = ConfigurationEvent { old: Some(old), new: self.configuration.clone() };

        self.change_handler.send(ChangeEvent::Configuration(configuration_event.clone()));
        self.configuration_handler.send(configuration_event);
    }
}
