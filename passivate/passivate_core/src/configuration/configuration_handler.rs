use crate::{actors::{ActorApi, Handler}, change_events::ChangeEvent};

use super::{ConfigurationEvent, PassivateConfig};


pub struct ConfigurationHandler {
    configuration: PassivateConfig,
    change_handler: ActorApi<ChangeEvent>
}

impl ConfigurationHandler {
    pub fn new(change_handler: ActorApi<ChangeEvent>) -> Self {
        Self { configuration: PassivateConfig::default(), change_handler }
    }

    pub fn configuration(&self) -> PassivateConfig {
        self.configuration.clone()
    }
}

impl Handler<ConfigurationEvent> for ConfigurationHandler {
    fn handle(&mut self, event: ConfigurationEvent, _cancellation: crate::actors::Cancellation) {
        match event {
            ConfigurationEvent::Update(passivate_config) => todo!(),
            ConfigurationEvent::Coverage(enabled) => {
                self.configuration.coverage_enabled = enabled;
                self.change_handler.send(ChangeEvent::Configuration(self.configuration.clone()));           
            }
        }
    }
}