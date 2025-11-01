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
    let mut layout = DockingLayout::new(vec![views.hyp_run_view().id()]);

    let state = layout.dock_state();

    let surface = state.main_surface_mut();

    let [_hyp_run_node, right_half] = surface.split_right(NodeIndex::root(), 0.4, vec![
        views.details_view().id(), 
        views.coverage_view().id(), 
        views.configuration_view().id()]);

    _ = surface.split_below(right_half, 0.8, vec![views.log_view().id()]);

    layout
}
