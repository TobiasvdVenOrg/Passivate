use std::fs;
use std::marker::PhantomData;
use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};

#[mockall::automock]
pub trait ConfigurationSource<T>
{
    fn load(&self) -> Result<Option<T>, ConfigurationLoadError>;

    fn persist(&self, configuration: &T) -> Result<(), ConfigurationPersistError>;
}

#[derive(Clone)]
pub struct FileConfigurationSource<T>
{
    path: Utf8PathBuf,
    _t_phantom: PhantomData<fn(T)>
}

impl<TPath, TConfiguration> From<TPath> for FileConfigurationSource<TConfiguration>
where
    TPath: AsRef<Utf8Path>
{
    fn from(value: TPath) -> Self
    {
        let path = value.as_ref().to_path_buf();

        FileConfigurationSource {
            path,
            _t_phantom: PhantomData
        }
    }
}

impl<T> FileConfigurationSource<T>
{
    fn load_file(path: &Utf8Path) -> Result<Option<T>, ConfigurationLoadError>
    where
        T: Serialize,
        T: DeserializeOwned
    {
        if !fs::exists(path).map_err(Arc::new)?
        {
            return Ok(None);
        }

        let content = fs::read(path).map_err(Arc::new)?;
        let text = String::from_utf8(content)?;
        let f = toml::from_str(&text)?;

        Ok(Some(f))
    }

    fn persist_file(path: &Utf8Path, content: String) -> Result<(), ConfigurationPersistError>
    {
        let dir = path.parent().ok_or(ConfigurationPersistError::Path(path.to_path_buf()))?;

        if !std::fs::exists(dir).map_err(Arc::new)?
        {
            std::fs::create_dir_all(dir).map_err(Arc::new)?;
        }

        Ok(std::fs::write(path, content).map_err(Arc::new)?)
    }
}

impl<T> ConfigurationSource<T> for FileConfigurationSource<T>
where
    T: Serialize,
    T: DeserializeOwned
{
    fn load(&self) -> Result<Option<T>, ConfigurationLoadError>
    {
        Self::load_file(&self.path)
    }

    fn persist(&self, configuration: &T) -> Result<(), ConfigurationPersistError>
    {
        let content = toml::to_string_pretty(configuration)?;

        Self::persist_file(&self.path, content)
    }
}

#[cfg(test)]
mod tests
{
    use camino::Utf8PathBuf;
    use galvanic_assert::matchers::collection::contains_in_order;
    use galvanic_assert::matchers::eq;
    use galvanic_assert::{assert_that, has_structure, structure};
    use passivate_testing::path_resolution::test_data_path;

    use crate::configuration::PassivateConfiguration;
    use crate::configuration_source::{ConfigurationSource, FileConfigurationSource};

    #[test]
    pub fn load_configuration_from_toml_file()
    {
        let file_path = test_data_path()
            .join("example_configurations")
            .join("minimal_configuration.toml");
        let source = FileConfigurationSource::from(file_path);
        let configuration = source.load().unwrap().unwrap();

        assert_that!(
            &configuration,
            has_structure!(PassivateConfiguration {
                coverage_enabled: eq(false),
                snapshot_directories: contains_in_order(vec![
                    Utf8PathBuf::from("a/path/to/snapshots"),
                    Utf8PathBuf::from("a/different/path/to/snapshots")
                ])
            })
        );
    }
}
