use std::sync::{Arc, Mutex, MutexGuard};

use passivate_delegation::{ActorEvent, Cancellation, Tx};

use super::{ConfigurationEvent, PassivateConfig};
use crate::change_events::ChangeEvent;

type TxConfiguration = Arc<dyn Tx<ConfigurationEvent>>;
type TxChangeEvent = Arc<dyn Tx<ActorEvent<ChangeEvent>>>;

#[derive(Clone)]
pub struct ConfigurationManager
{
    configuration: Arc<Mutex<PassivateConfig>>,
    configuration_tx: TxConfiguration,
    change_event_tx: TxChangeEvent
}

impl ConfigurationManager
{
    pub fn new(configuration: PassivateConfig, configuration_tx: TxConfiguration, change_event_tx: TxChangeEvent) -> Self
    {
        Self {
            configuration: Arc::new(Mutex::new(configuration)),
            configuration_tx,
            change_event_tx
        }
    }

    pub fn default_config(configuration_tx: TxConfiguration, change_event_tx: TxChangeEvent) -> Self
    {
        Self::new(PassivateConfig::default(), configuration_tx, change_event_tx)
    }

    pub fn update<TUpdater: Fn(&mut PassivateConfig)>(&mut self, updater: TUpdater)
    {
        let mut configuration = self.acquire();

        let old = configuration.clone();

        updater(&mut configuration);

        let new = configuration.clone();

        drop(configuration);

        self.configuration_tx.send(ConfigurationEvent { old, new });
        self.change_event_tx.send(ActorEvent {
            event: ChangeEvent::DefaultRun,
            cancellation: Cancellation::default()
        });
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
