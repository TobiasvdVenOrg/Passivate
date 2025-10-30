use camino::Utf8Path;
use egui_dock::NodeIndex;
use passivate_configuration::configuration_errors::ConfigurationLoadError;

use crate::docking::docking_layout::DockingLayout;
use crate::docking::layout_management::LayoutManagement;
use crate::docking::view::View;
use crate::passivate_views::PassivateViews;

pub fn load(
    path: &Utf8Path,
    views: &PassivateViews
) -> Result<LayoutManagement, ConfigurationLoadError>
{
    LayoutManagement::from_file_or_default(path, || {
        default(views)
    })
}

pub fn default(views: &PassivateViews) -> DockingLayout
{
    let mut layout = DockingLayout::new(views.ids().into_iter().skip(1).collect());

    let state = layout.dock_state();

    let surface = state.main_surface_mut();

    let [_old, _new] = surface.split_left(NodeIndex::root(), 0.4, vec![views.hyp_run_view().id()]);

    layout
}
