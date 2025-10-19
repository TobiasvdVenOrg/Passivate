use camino::Utf8Path;
use passivate_configuration::configuration_errors::ConfigurationLoadError;
use passivate_configuration::configuration_source::ConfigurationSource;

use crate::docking::docking_layout::DockingLayout;

#[derive(Debug)]
pub struct LayoutManagement
{
    current_layout: DockingLayout,
    source: Option<ConfigurationSource<DockingLayout>>
}

impl LayoutManagement
{
    pub fn from_file_or_default<FDefault>(path: &Utf8Path, default: FDefault) -> Result<Self, ConfigurationLoadError>
    where
        FDefault: FnOnce() -> DockingLayout
    {
        let source = ConfigurationSource::from_file(path);

        Self::from_source_or_default(source, default)
    }

    pub fn from_source_or_default<FDefault>(
        source: ConfigurationSource<DockingLayout>,
        default: FDefault
    ) -> Result<Self, ConfigurationLoadError>
    where
        FDefault: FnOnce() -> DockingLayout
    {
        if let Some(layout) = source.load()?
        {
            Ok(Self {
                current_layout: layout,
                source: Some(source)
            })
        }
        else
        {
            Ok(Self {
                current_layout: default(),
                source: Some(source)
            })
        }
    }

    pub fn show_current(&mut self, egui_context: &egui::Context, tab_viewer: &mut super::tab_viewer::TabViewer)
    {
        self.current_layout.show(egui_context, tab_viewer);
    }
}

impl Drop for LayoutManagement
{
    fn drop(&mut self)
    {
        if let Some(source) = &self.source
        {
            let _error = source.persist(&self.current_layout).map_err(|error| {
                log::error!("failed to persist docking layout: {:?}", error);
            });
        }
    }
}
