use std::{fs, marker::PhantomData, sync::Arc};

use camino::{Utf8Path, Utf8PathBuf};
use serde::{de::DeserializeOwned, Serialize};

use crate::configuration_errors::{ConfigurationLoadError, ConfigurationPersistError};

#[faux::create]
#[derive(Clone)]
pub struct ConfigurationSource<T>
{
    kind: ConfigurationSourceKind,
    _t: PhantomData<T>
}

#[derive(Clone)]
pub enum ConfigurationSourceKind
{
    File(Utf8PathBuf)
}

#[faux::methods]
impl<T> ConfigurationSource<T>
where
    T : Serialize, 
    T : DeserializeOwned
{
    pub fn from_file<P: AsRef<Utf8Path>>(file_path: P) -> ConfigurationSource<T> {
        let path = file_path.as_ref().to_path_buf();
        
        Self {
            kind: ConfigurationSourceKind::File(path), _t: PhantomData
        }
    }

    pub fn load(&self) -> Result<Option<T>, ConfigurationLoadError>
    {
        match &self.kind
        {
            ConfigurationSourceKind::File(path) => Self::load_file(path),
        }
    }

    pub fn persist(&self, configuration: &T) -> Result<(), ConfigurationPersistError>
    {
        let content = toml::to_string_pretty(configuration)?;

        match &self.kind
        {
            ConfigurationSourceKind::File(path) => Self::persist_file(path.as_path(), content),
        }
    }

    fn load_file(path: &Utf8Path) -> Result<Option<T>, ConfigurationLoadError>
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

#[cfg(test)]
mod tests
{
    use galvanic_assert::matchers::eq;
    use galvanic_assert::{assert_that, has_structure, structure};
    use passivate_testing::path_resolution::test_data_path;

    use crate::configuration::PassivateConfiguration;
    use crate::configuration_source::ConfigurationSource;

    #[test]
    pub fn load_configuration_from_toml_file()
    {
        let file_path = test_data_path().join("example_configurations").join("minimal_configuration.toml");
        let source = ConfigurationSource::from_file(file_path);
        let configuration = source.load().unwrap().unwrap();

        assert_that!(
            &configuration,
            has_structure!(PassivateConfiguration {
                coverage_enabled: eq(false),
                snapshots_path: eq(Some("a/path/to/snapshots".to_string()))
            })
        );
    }
}