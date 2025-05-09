use std::sync::{Arc, Mutex, MutexGuard};

use passivate_delegation::Tx;

use super::{ConfigurationEvent, PassivateConfig};


pub struct ConfigurationManager {
    configuration: Arc<Mutex<PassivateConfig>>,
    configuration_tx: Tx<ConfigurationEvent>
}

impl ConfigurationManager {
    pub fn new(configuration: PassivateConfig, configuration_tx: Tx<ConfigurationEvent>) -> Self {
        Self { configuration: Arc::new(Mutex::new(configuration)), configuration_tx }
    }

    pub fn update<TUpdater: Fn(&mut PassivateConfig)>(&mut self, updater: TUpdater) {
        let mut configuration = self.acquire();

        let old = configuration.clone();

        updater(&mut configuration);

        let new = configuration.clone();

        drop(configuration);
        self.configuration_tx.send(ConfigurationEvent { old, new });
    }

    pub fn snapshots_path(&self) -> Option<String> {
        self.get(|c| c.snapshots_path.clone())
    }

    fn get<TValue, TGet: Fn(&PassivateConfig) -> TValue>(&self, get: TGet) -> TValue {
        let configuration = self.acquire();
        get(&configuration)
    }

    fn acquire(&self) -> MutexGuard<'_, PassivateConfig> {
        self.configuration.lock().expect("failed to acquire configuration lock.")
    }
}
