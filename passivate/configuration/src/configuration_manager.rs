use std::sync::{Arc, Mutex, MutexGuard};

use crate::configuration::{ConfigurationChange, PassivateConfiguration};
use crate::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};
use crate::configuration_source::ConfigurationSource;

#[derive(Clone)]
pub struct ConfigurationManager
{
    configuration: Arc<Mutex<PassivateConfiguration>>,
    source: Option<Arc<dyn ConfigurationSource<PassivateConfiguration> + Send + Sync + 'static>>
}

impl ConfigurationManager
{
    pub fn new(configuration: PassivateConfiguration) -> ConfigurationManager
    {
        ConfigurationManager {
            configuration: Arc::new(Mutex::new(configuration)),
            source: None
        }
    }

    pub fn from_source<TSource>(source: TSource) -> Result<Self, ConfigurationLoadError>
    where
        TSource: ConfigurationSource<PassivateConfiguration> + Send + Sync + 'static
    {
        let configuration = source.load()?.unwrap_or_default();
        let source = Arc::new(source);

        Ok(Self {
            configuration: Arc::new(Mutex::new(configuration)),
            source: Some(source)
        })
    }

    pub fn change(&mut self, change: ConfigurationChange) -> Result<(), ConfigurationPersistError>
    {
        let mut configuration = self.acquire();

        configuration.change(change);

        if let Some(source) = &self.source
        {
            source.persist(&configuration).map_err(|error| {
                log::error!("failed to persist configuration: {:?}", error);
                error
            })?
        }

        Ok(())
    }

    pub fn get<TValue, TGet: Fn(&PassivateConfiguration) -> TValue>(&self, get: TGet) -> TValue
    {
        let configuration = self.acquire();
        get(&configuration)
    }

    pub fn acquire(&self) -> MutexGuard<'_, PassivateConfiguration>
    {
        self.configuration.lock().expect("failed to acquire configuration lock.")
    }
}

impl Default for ConfigurationManager
{
    fn default() -> Self
    {
        Self::new(PassivateConfiguration::default())
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
    use passivate_hyp_names::test_name;
    use passivate_testing::path_resolution::{clean_directory, copy_from_data_to_output, test_output_path};
    use passivate_testing::spy_log::SpyLog;

    use crate::configuration::{ConfigurationChange, PassivateConfiguration};
    use crate::configuration_errors::ConfigurationPersistError;
    use crate::configuration_manager::ConfigurationManager;
    use crate::configuration_source::{FileConfigurationSource, MockConfigurationSource};

    #[test]
    pub fn when_configuration_is_loaded_from_file_then_changes_are_persisted_there() -> Result<(), Box<dyn Error>>
    {
        let file = copy_from_data_to_output("example_configurations/minimal_configuration.toml")?;

        let mut manager = ConfigurationManager::from_source(FileConfigurationSource::from(&file))?;

        assert_that!(&manager.get(|c| c.coverage_enabled), eq(false));
        manager.change(ConfigurationChange::CoverageEnabled(true)).unwrap();

        let reloaded_manager = ConfigurationManager::from_source(FileConfigurationSource::from(&file))?;

        assert_that!(&reloaded_manager.get(|c| c.coverage_enabled), eq(true));
        Ok(())
    }

    #[test]
    pub fn when_configuration_is_persisted_for_the_first_time_a_file_is_created() -> Result<(), Box<dyn Error>>
    {
        let dir = test_output_path().join(test_name!());

        clean_directory(&dir);

        let file = dir.join("new_configuration.toml");
        let source = FileConfigurationSource::from(&file);

        let mut manager = ConfigurationManager::from_source(source)?;

        manager
            .change(ConfigurationChange::AddSnapshotDirectory(Utf8PathBuf::from("first/change")))
            .unwrap();

        assert!(fs::exists(file)?);
        Ok(())
    }

    #[test]
    pub fn an_error_is_logged_when_configuration_persistence_fails() -> Result<(), Box<dyn Error>>
    {
        let mut source = MockConfigurationSource::new();

        source.expect_load().returning(|| Ok(None));
        source.expect_persist().returning(|_| {
            Err(ConfigurationPersistError::Io(Arc::new(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                ""
            ))))
        });

        let mut manager = ConfigurationManager::from_source(source)?;

        let spy_log = SpyLog::set();

        manager.change(ConfigurationChange::CoverageEnabled(true)).unwrap_err();

        let error = spy_log.into_iter().exactly_one().unwrap();

        assert_that!(error.starts_with("ERROR"));

        Ok(())
    }

    #[test]
    pub fn configuration_update_changes_configuration()
    {
        let configuration = PassivateConfiguration::default();
        let mut manager = ConfigurationManager::new(configuration);

        manager
            .change(ConfigurationChange::AddSnapshotDirectory(Utf8PathBuf::from("Example/path")))
            .unwrap();

        let snapshots_path = manager.get(|c| c.snapshot_directories.iter().exactly_one().unwrap().clone());

        assert_eq!(Utf8PathBuf::from("Example/path"), snapshots_path);
    }
}
