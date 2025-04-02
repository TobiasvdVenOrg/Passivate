use crate::{actors::{ActorApi, Handler}, change_events::ChangeEvent};

use super::{ConfigurationChangeEvent, ConfigurationEvent, PassivateConfig};


pub struct ConfigurationHandler {
    configuration: PassivateConfig,
    change_handler: ActorApi<ChangeEvent>,
    sender: crossbeam_channel::Sender<ConfigurationEvent>
}

impl ConfigurationHandler {
    pub fn new(change_handler: ActorApi<ChangeEvent>, sender: crossbeam_channel::Sender<ConfigurationEvent>) -> Self {
        Self { configuration: PassivateConfig::default(), change_handler, sender }
    }

    pub fn configuration(&self) -> PassivateConfig {
        self.configuration.clone()
    }
}

impl Handler<ConfigurationChangeEvent> for ConfigurationHandler {
    fn handle(&mut self, event: ConfigurationChangeEvent, _cancellation: crate::actors::Cancellation) {
        match event {
            ConfigurationChangeEvent::Coverage(enabled) => {
                let old = self.configuration.clone();
                self.configuration.coverage_enabled = enabled;
                self.change_handler.send(ChangeEvent::Configuration(self.configuration.clone()));
                let _ = self.sender.send(ConfigurationEvent { old: Some(old), new: self.configuration.clone() });
            }
            ConfigurationChangeEvent::SnapshotsPath(snapshots_path) => {
                let old = self.configuration.clone();
                self.configuration.snapshots_path = Some(snapshots_path);
                self.change_handler.send(ChangeEvent::Configuration(self.configuration.clone()));
                let _ = self.sender.send(ConfigurationEvent { old: Some(old), new: self.configuration.clone() });
            }
        }
    }
}