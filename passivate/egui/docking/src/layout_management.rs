use camino::Utf8Path;
use passivate_configuration::configuration_errors::ConfigurationLoadError;
use passivate_configuration::configuration_source::{ConfigurationSource, FileConfigurationSource};

use crate::docking_layout::DockingLayout;

#[derive(Debug)]
pub struct LayoutManagement<TSource>
where
    TSource: ConfigurationSource<DockingLayout>
{
    current_layout: DockingLayout,
    source: TSource
}

impl<TSource> LayoutManagement<TSource>
where
    TSource: ConfigurationSource<DockingLayout>
{
    pub fn from_file_or_default<FDefault>(
        path: &Utf8Path,
        default: FDefault
    ) -> Result<LayoutManagement<FileConfigurationSource<DockingLayout>>, ConfigurationLoadError>
    where
        FDefault: FnOnce() -> DockingLayout
    {
        let source = FileConfigurationSource::from(path);

        LayoutManagement::<FileConfigurationSource<DockingLayout>>::from_source_or_default(source, default)
    }

    pub fn from_source_or_default<FDefault>(source: TSource, default: FDefault) -> Result<Self, ConfigurationLoadError>
    where
        FDefault: FnOnce() -> DockingLayout
    {
        if let Some(layout) = source.load()?
        {
            Ok(Self {
                current_layout: layout,
                source
            })
        }
        else
        {
            Ok(Self {
                current_layout: default(),
                source
            })
        }
    }

    pub fn get_current(&mut self) -> &mut DockingLayout
    {
        &mut self.current_layout
    }
}

impl<TSource> Drop for LayoutManagement<TSource>
where
    TSource: ConfigurationSource<DockingLayout>
{
    fn drop(&mut self)
    {
        let _error = self.source.persist(&self.current_layout).map_err(|error| {
            log::error!("failed to persist docking layout: {:?}", error);
        });
    }
}
