use std::sync::{Arc, Mutex, MutexGuard};

use crossbeam_channel::Sender;
use passivate_delegation::Tx;

use crate::change_events::ChangeEvent;

use super::{ConfigurationEvent, PassivateConfig};

#[derive(Clone)]
pub struct ConfigurationManager
{
    configuration: Arc<Mutex<PassivateConfig>>,
    configuration_tx: Sender<ConfigurationEvent>,
    change_event_tx: Sender<ChangeEvent>
}

impl ConfigurationManager
{
    pub fn new(configuration: PassivateConfig, configuration_tx: Sender<ConfigurationEvent>, change_event_tx: Sender<ChangeEvent>) -> Self
    {
        Self {
            configuration: Arc::new(Mutex::new(configuration)),
            configuration_tx,
            change_event_tx
        }
    }

    pub fn update<TUpdater: Fn(&mut PassivateConfig)>(&mut self, updater: TUpdater)
    {
        let mut configuration = self.acquire();

        let old = configuration.clone();

        updater(&mut configuration);

        let new = configuration.clone();

        drop(configuration);

        self.configuration_tx.send(ConfigurationEvent { old, new }).expect("failed to send configuration event");
        self.change_event_tx.send(ChangeEvent::DefaultRun).expect("failed to send change event");
    }

    pub fn get_copy(&self) -> PassivateConfig
    {
        let configuration = self.acquire();

        configuration.clone()
    }

    pub fn get<TValue, TGet: Fn(&PassivateConfig) -> TValue>(&self, get: TGet) -> TValue
    {
        let configuration = self.acquire();
        get(&configuration)
    }

    fn acquire(&self) -> MutexGuard<'_, PassivateConfig>
    {
        self.configuration.lock().expect("failed to acquire configuration lock.")
    }
}
