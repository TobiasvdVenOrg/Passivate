use camino::Utf8Path;
use passivate_configuration::configuration_errors::ConfigurationLoadError;

use crate::docking::docking_layout::DockingLayout;
use crate::docking::layout_management::LayoutManagement;
use crate::passivate_views::PassivateViews;

pub fn load(
    path: &Utf8Path,
    views: &PassivateViews
) -> Result<LayoutManagement, ConfigurationLoadError>
{
    LayoutManagement::from_file_or_default(path, ||
    {
        default(views)
    })
}

pub fn default(views: &PassivateViews) -> DockingLayout
{
    DockingLayout::new(views.ids().into_iter().collect())
}