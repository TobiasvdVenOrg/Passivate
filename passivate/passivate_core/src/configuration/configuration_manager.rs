use std::sync::{Arc, Mutex, MutexGuard};

use passivate_configuration::configuration::Configuration;
use passivate_delegation::Tx;

use super::ConfigurationEvent;

#[derive(Clone)]
pub struct ConfigurationManager
{
    configuration: Arc<Mutex<Configuration>>,
    configuration_tx: Tx<ConfigurationEvent>
}

impl ConfigurationManager
{
    pub fn new(configuration: Configuration, configuration_tx: Tx<ConfigurationEvent>) -> Self
    {
        Self {
            configuration: Arc::new(Mutex::new(configuration)),
            configuration_tx
        }
    }

    pub fn default_config(configuration_tx: Tx<ConfigurationEvent>) -> Self
    {
        Self::new(Configuration::default(), configuration_tx)
    }

    pub fn update<TUpdater: Fn(&mut Configuration)>(&mut self, updater: TUpdater)
    {
        let mut configuration = self.acquire();

        let old = configuration.clone();

        updater(&mut configuration);

        let new = configuration.clone();

        drop(configuration);

        self.configuration_tx.send(ConfigurationEvent { old, new });
    }

    pub fn get_copy(&self) -> Configuration
    {
        let configuration = self.acquire();

        configuration.clone()
    }

    pub fn get<TValue, TGet: Fn(&Configuration) -> TValue>(&self, get: TGet) -> TValue
    {
        let configuration = self.acquire();
        get(&configuration)
    }

    fn acquire(&self) -> MutexGuard<'_, Configuration>
    {
        self.configuration.lock().expect("failed to acquire configuration lock.")
    }
}
