use std::sync::mpsc::Sender;

use crate::{actors::{ActorApi, Handler}, change_events::ChangeEvent};

use super::{ConfigurationChangeEvent, PassivateConfig};


pub struct ConfigurationHandler {
    configuration: PassivateConfig,
    change_handler: ActorApi<ChangeEvent>,
    sender: Sender<PassivateConfig>
}

impl ConfigurationHandler {
    pub fn new(change_handler: ActorApi<ChangeEvent>, sender: Sender<PassivateConfig>) -> Self {
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
                self.configuration.coverage_enabled = enabled;
                self.change_handler.send(ChangeEvent::Configuration(self.configuration.clone()));
                let _ = self.sender.send(self.configuration.clone());
            }
        }
    }
}