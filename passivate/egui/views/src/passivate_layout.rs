use camino::Utf8Path;
use egui_dock::NodeIndex;
use passivate_configuration::configuration_errors::ConfigurationLoadError;
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_docking::layout_management::LayoutManagement;
use passivate_egui_docking::view::View;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_bridge::HypRunBridge;

use crate::passivate_views::PassivateViews;

pub fn load<TBridge: Bridge>(
    path: &Utf8Path,
    views: &PassivateViews<TBridge, TBridge::HypRunner>
) -> Result<LayoutManagement, ConfigurationLoadError>
{
    LayoutManagement::from_file_or_default(path, || default(views))
}

pub fn default<TBridge: Bridge, THypRunBridge: HypRunBridge>(views: &PassivateViews<TBridge, THypRunBridge>) -> DockingLayout
{
    let mut layout = DockingLayout::new(vec![views.session_dock().id()]);

    let state = layout.dock_state();

    let surface = state.main_surface_mut();

    let [_hyp_run_node, right_half] = surface.split_right(
        NodeIndex::root(),
        0.4,
        vec![
            views.details_dock().id(),
            views.coverage_dock().id(),
            views.configuration_dock().id(),
        ]
    );

    _ = surface.split_below(right_half, 0.8, vec![views.log_dock().id()]);

    layout
}
