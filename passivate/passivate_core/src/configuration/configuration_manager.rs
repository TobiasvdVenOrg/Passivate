use std::sync::{Arc, Mutex, MutexGuard};

use passivate_delegation::Tx;

use crate::change_events::ChangeEvent;

use super::{ConfigurationEvent, PassivateConfig};

#[derive(Clone)]
pub struct ConfigurationManager
{
    configuration: Arc<Mutex<PassivateConfig>>,
    configuration_tx: Arc<Mutex<Tx<ConfigurationEvent>>>,
    change_event_tx: Arc<Mutex<Tx<ChangeEvent>>>
}

impl ConfigurationManager
{
    pub fn new(configuration: PassivateConfig, configuration_tx: Tx<ConfigurationEvent>, change_event_tx: Tx<ChangeEvent>) -> Self
    {
        Self {
            configuration: Arc::new(Mutex::new(configuration)),
            configuration_tx: Arc::new(Mutex::new(configuration_tx)),
            change_event_tx: Arc::new(Mutex::new(change_event_tx))
        }
    }

    pub fn update<TUpdater: Fn(&mut PassivateConfig)>(&mut self, updater: TUpdater)
    {
        let mut configuration = self.acquire();

        let old = configuration.clone();

        updater(&mut configuration);

        let new = configuration.clone();

        drop(configuration);

        self.configuration_tx.lock().expect("failed to acquire configuration event tx lock.").send(ConfigurationEvent { old, new });
        self.change_event_tx.lock().expect("failed to acquire change event tx lock.").send(ChangeEvent::DefaultRun);
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
