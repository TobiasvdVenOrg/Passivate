use camino::Utf8Path;
use passivate_configuration::configuration_errors::ConfigurationLoadError;

use crate::docking::docking_layout::DockingLayout;
use crate::docking::layout_management::LayoutManagement;
use crate::docking::view::View;
use crate::passivate_view::PassivateView;

pub fn load(
    path: &Utf8Path,
    views: &[PassivateView]
) -> Result<LayoutManagement, ConfigurationLoadError>
{
    LayoutManagement::from_file_or_default(path, ||
    {
        default(views)
    })
}

pub fn default(views: &[PassivateView]) -> DockingLayout
{
    DockingLayout::new(views.iter().map(|view| view.id()))
}