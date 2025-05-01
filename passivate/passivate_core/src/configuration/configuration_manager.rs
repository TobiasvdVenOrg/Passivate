use std::sync::{Arc, Mutex, MutexGuard};

use super::PassivateConfig;


pub struct ConfigurationManager {
    configuration: Arc<Mutex<PassivateConfig>>
}

impl ConfigurationManager {
    pub fn new(configuration: PassivateConfig) -> Self {
        Self { configuration: Arc::new(Mutex::new(configuration)) }
    }

    pub fn update<TUpdater: Fn(&mut PassivateConfig)>(&mut self, updater: TUpdater) {
        let mut configuration = self.acquire();
        updater(&mut configuration);
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
