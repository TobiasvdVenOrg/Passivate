use std::sync::{Arc, Mutex, MutexGuard};

use passivate_delegation::Tx;

use crate::{configuration::Configuration, configuration_event::ConfigurationEvent};

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

mod tests
{
    use passivate_delegation::Tx;

    use crate::{configuration::Configuration, configuration_manager::ConfigurationManager};

    #[test]
    pub fn configuration_update_changes_configuration()
    {
        let configuration = Configuration::default();
        let mut manager = ConfigurationManager::new(configuration, Tx::stub());

        manager.update(|c| {
            c.snapshots_path = Some(String::from("Example/path"));
        });

        let snapshots_path = manager.get(|c| c.snapshots_path.clone());

        assert_eq!(Some("Example/path"), snapshots_path.as_deref());
    }

    #[test]
    pub fn configuration_change_is_broadcast()
    {
        let configuration = Configuration::default();
        let (tx, rx1, rx2) = Tx::multi_2();
        let mut manager = ConfigurationManager::new(configuration, tx);

        manager.update(|c| {
            c.coverage_enabled = true;
        });

        let broadcast1 = rx1.last().unwrap();
        let broadcast2 = rx2.last().unwrap();

        assert_eq!(broadcast1, broadcast2);
    }
}