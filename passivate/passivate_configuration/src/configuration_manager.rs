use std::sync::{Arc, Mutex, MutexGuard};

use camino::Utf8Path;
use passivate_delegation::Tx;

use crate::configuration::{PassivateConfiguration};
use crate::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};
use crate::configuration_event::ConfigurationEvent;
use crate::configuration_source::ConfigurationSource;

#[derive(Clone)]
pub struct ConfigurationManager
{
    configuration: Arc<Mutex<PassivateConfiguration>>,
    configuration_tx: Tx<ConfigurationEvent>,
    source: Option<ConfigurationSource<PassivateConfiguration>>
}

impl ConfigurationManager
{
    pub fn from_source(configuration_tx: Tx<ConfigurationEvent>, source: ConfigurationSource<PassivateConfiguration>) -> Result<Self, ConfigurationLoadError>
    {
        let configuration = source.load()?.unwrap_or_default();

        Ok(Self {
            configuration: Arc::new(Mutex::new(configuration)),
            configuration_tx,
            source: Some(source)
        })
    }

    pub fn from_file<P: AsRef<Utf8Path>>(configuration_tx: Tx<ConfigurationEvent>, file_path: P) -> Result<Self, ConfigurationLoadError>
    {
        let source = ConfigurationSource::from_file(file_path);

        Self::from_source(configuration_tx, source)
    }

    pub fn new(configuration: PassivateConfiguration, configuration_tx: Tx<ConfigurationEvent>) -> Self
    {
        Self {
            configuration: Arc::new(Mutex::new(configuration)),
            configuration_tx,
            source: None
        }
    }

    pub fn default_config(configuration_tx: Tx<ConfigurationEvent>) -> Self
    {
        Self::new(PassivateConfiguration::default(), configuration_tx)
    }

    pub fn update<TUpdater: Fn(&mut PassivateConfiguration)>(&mut self, updater: TUpdater) -> Result<(), ConfigurationPersistError>
    {
        let mut configuration = self.acquire();

        let old = configuration.clone();

        updater(&mut configuration);

        let new = configuration.clone();

        drop(configuration);

        if let Some(source) = &self.source
        {
            source.persist(&new).map_err(|error| 
            {
                log::error!("failed to persist configuration: {:?}", error);
                error
            })?
        }

        self.configuration_tx.send(ConfigurationEvent { old, new });

        Ok(())
    }

    pub fn get_copy(&self) -> PassivateConfiguration
    {
        let configuration = self.acquire();

        configuration.clone()
    }

    pub fn get<TValue, TGet: Fn(&PassivateConfiguration) -> TValue>(&self, get: TGet) -> TValue
    {
        let configuration = self.acquire();
        get(&configuration)
    }

    fn acquire(&self) -> MutexGuard<'_, PassivateConfiguration>
    {
        self.configuration.lock().expect("failed to acquire configuration lock.")
    }
}

#[cfg(test)]
mod tests
{
    use std::error::Error;
    use std::fs;
    use std::sync::Arc;

    use camino::Utf8PathBuf;
    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::eq;
    use itertools::Itertools;
    use passivate_delegation::Tx;
    use passivate_testing::path_resolution::{copy_from_data_to_output, test_output_path};
    use passivate_testing::spy_log::SpyLog;

    use crate::configuration::PassivateConfiguration;
    use crate::configuration_errors::ConfigurationPersistError;
    use crate::configuration_manager::ConfigurationManager;
    use crate::configuration_source::ConfigurationSource;

    #[test]
    pub fn when_configuration_is_loaded_from_file_then_changes_are_persisted_there() -> Result<(), Box<dyn Error>>
    {
        let file = copy_from_data_to_output("example_configurations/minimal_configuration.toml")?;

        let mut manager = ConfigurationManager::from_file(Tx::stub(), &file)?;

        assert_that!(&manager.get(|c| c.coverage_enabled), eq(false));
        manager.update(|c| c.coverage_enabled = true).unwrap();

        let reloaded_manager = ConfigurationManager::from_file(Tx::stub(), &file)?;

        assert_that!(&reloaded_manager.get(|c| c.coverage_enabled), eq(true));
        Ok(())
    }

    #[test]
    pub fn when_configuration_is_persisted_for_the_first_time_a_file_is_created() -> Result<(), Box<dyn Error>>
    {
        let file = test_output_path().join("example_configurations/new_configuration.toml");

        let mut manager = ConfigurationManager::from_file(Tx::stub(), file.as_path())?;

        manager.update(|c| c.snapshot_directories.push(Utf8PathBuf::from("first/change"))).unwrap();

        assert!(fs::exists(file)?);
        Ok(())
    }

    #[test]
    pub fn an_error_is_logged_when_configuration_persistence_fails() -> Result<(), Box<dyn Error>>
    {
        let mut source = ConfigurationSource::faux();

        source._when_load().then_return(Ok(None));
        source._when_persist().then_return(Err(ConfigurationPersistError::Io(Arc::new(std::io::Error::new(std::io::ErrorKind::Interrupted, "")))));

        let mut manager = ConfigurationManager::from_source(Tx::stub(), source)?;

        let spy_log = SpyLog::set();

        manager.update(|c| c.coverage_enabled = true).unwrap_err();

        let error = spy_log.into_iter().exactly_one().unwrap();

        assert_that!(error.starts_with("ERROR"));
        
        Ok(())
    }

    #[test]
    pub fn configuration_update_changes_configuration()
    {
        let configuration = PassivateConfiguration::default();
        let mut manager = ConfigurationManager::new(configuration, Tx::stub());

        manager.update(|c| c.snapshot_directories.push(Utf8PathBuf::from("Example/path"))).unwrap();

        let snapshots_path = manager.get(|c| c.snapshot_directories.iter().exactly_one().unwrap().clone());

        assert_eq!(Utf8PathBuf::from("Example/path"), snapshots_path);
    }

    #[test]
    pub fn configuration_change_is_broadcast()
    {
        let configuration = PassivateConfiguration::default();
        let (tx, rx1, rx2) = Tx::multi_2();
        let mut manager = ConfigurationManager::new(configuration, tx);

        manager.update(|c| c.coverage_enabled = true).unwrap();

        let broadcast1 = rx1.drain().last().unwrap().clone();
        let broadcast2 = rx2.drain().last().unwrap().clone();

        assert_eq!(broadcast1, broadcast2);
    }
}
