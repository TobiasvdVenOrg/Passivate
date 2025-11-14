use egui_dock::{DockArea, Style};
use passivate_core::passivate_state::PassivateState;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_egui_core::PassivateViewState;
use passivate_egui_docking::dock_views::{DockViewer, DockViews};
use passivate_egui_docking::docking_layout::DockingLayout;

use crate::passivate_views::PassivateView;

pub struct PassivateViewContext<'a, 'b>
{
    view_state: &'a PassivateViewState,
    state: &'b PassivateState,
    changes: Vec<PassivateStateChange<'b>>
}

impl<'a, 'b> PassivateViewContext<'a, 'b>
{
    fn new(view_state: &'a PassivateViewState, state: &'b PassivateState) -> Self
    {
        Self {
            state,
            view_state,
            changes: Vec::new()
        }
    }
}

pub fn ui<'b>(
    view_state: &PassivateViewState,
    state: &'b PassivateState,
    egui_context: &egui::Context,
    dock_views: &mut DockViews<PassivateView>,
    layout: &mut DockingLayout
) -> Vec<PassivateStateChange<'b>>
{
    let mut passivate_context = PassivateViewContext::new(view_state, state);

    let mut dock_viewer = DockViewer {
        dock_views,
        context: &mut passivate_context,
        custom_ui: internal_ui
    };

    DockArea::new(layout.dock_state())
        .style(Style::from_egui(egui_context.style().as_ref()))
        .show_close_buttons(false)
        .show_leaf_collapse_buttons(false)
        .show_leaf_close_all_buttons(false)
        .show(egui_context, &mut dock_viewer);

    egui_context.request_repaint();

    passivate_context.changes
}

fn internal_ui<'a, 'b>(ui: &mut egui::Ui, view: &mut PassivateView, context: &mut PassivateViewContext<'a, 'b>)
{
    let change = {
        let state = context.state;
        let view_state = context.view_state;

        match view
        {
            PassivateView::Configuration(configuration_view) =>
            {
                configuration_view.ui(ui);
                None
            }
            PassivateView::Coverage(coverage_view) =>
            {
                coverage_view.ui(ui);
                None
            }
            PassivateView::Details(details_view) =>
            {
                details_view.ui(ui, view_state.hyp_details.as_ref());
                None
            }
            PassivateView::Log(log_view) =>
            {
                log_view.ui(ui);
                None
            }
            PassivateView::HypRun(test_run_view) =>
            {
                test_run_view
                    .ui(ui, &state.hyp_session)
                    .map(PassivateStateChange::HypSelected)
            }
        }
    };

    if let Some(change) = change
    {
        context.changes.push(change);
    }
}
