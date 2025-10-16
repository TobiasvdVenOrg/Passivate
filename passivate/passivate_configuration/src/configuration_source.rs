use std::sync::Arc;

use camino::{Utf8Path, Utf8PathBuf};

use crate::configuration::{Configuration, ConfigurationLoadError, ConfigurationPersistError};

#[faux::create]
#[derive(Clone)]
pub struct ConfigurationSource
{
    kind: ConfigurationSourceKind
}

#[derive(Clone)]
pub enum ConfigurationSourceKind
{
    File(Utf8PathBuf)
}

#[faux::methods]
impl ConfigurationSource
{
    pub fn from_file<P: AsRef<Utf8Path>>(file_path: P) -> ConfigurationSource {
        let path = file_path.as_ref().to_path_buf();
        
        Self {
            kind: ConfigurationSourceKind::File(path)
        }
    }

    pub fn load(&self) -> Result<Configuration, ConfigurationLoadError>
    {
        match &self.kind
        {
            ConfigurationSourceKind::File(path) => load_file(path),
        }
    }

    pub fn persist(&self, configuration: &Configuration) -> Result<(), ConfigurationPersistError>
    {
        let content = toml::to_string_pretty(configuration)?;

        match &self.kind
        {
            ConfigurationSourceKind::File(path) => persist_file(path.as_path(), content),
        }
    }
}

fn load_file(path: &Utf8Path) -> Result<Configuration, ConfigurationLoadError>
{
    let content = std::fs::read(path).map_err(Arc::new)?;
    let text = String::from_utf8(content)?;
    
    Ok(toml::from_str(&text)?)
}

fn persist_file(path: &Utf8Path, content: String) -> Result<(), ConfigurationPersistError>
{
    Ok(std::fs::write(path, content).map_err(Arc::new)?)
}

#[cfg(test)]
mod tests
{
    use galvanic_assert::matchers::eq;
    use galvanic_assert::{assert_that, has_structure, structure};
    use passivate_testing::path_resolution::test_data_path;

    use crate::configuration::Configuration;
    use crate::configuration_source::ConfigurationSource;

    #[test]
    pub fn load_configuration_from_toml_file()
    {
        let file_path = test_data_path().join("example_configurations").join("minimal_configuration.toml");
        let source = ConfigurationSource::from_file(file_path);
        let configuration = source.load().unwrap();

        assert_that!(
            &configuration,
            has_structure!(Configuration {
                coverage_enabled: eq(false),
                snapshots_path: eq(Some("a/path/to/snapshots".to_string()))
            })
        );
    }
}