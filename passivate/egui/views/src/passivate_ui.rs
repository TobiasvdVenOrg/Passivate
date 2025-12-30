use egui_dock::{DockArea, Style};
use passivate_core::passivate_state::PassivateState;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::{DockViewer, DockViews};
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_view_details::hyp_details::HypDetails;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_bridge::HypRunBridge;

use crate::passivate_views::PassivateView;

pub struct PassivateViewContext<'a, 'b, TBridge: Bridge>
{
    view_state: &'a PassivateViewState<TBridge>,
    state: &'b PassivateState<TBridge>,
    changes: Vec<PassivateStateChange<'b, TBridge>>
}

impl<'a, 'b, TBridge: Bridge> PassivateViewContext<'a, 'b, TBridge>
{
    fn new(view_state: &'a PassivateViewState<TBridge>, state: &'b PassivateState<TBridge>) -> Self
    {
        Self {
            state,
            view_state,
            changes: Vec::new()
        }
    }
}

pub fn ui<'b, TBridge: Bridge, THypRunBridge: HypRunBridge>(
    view_state: &PassivateViewState<TBridge>,
    state: &'b PassivateState<TBridge>,
    egui_context: &egui::Context,
    dock_views: &mut DockViews<PassivateView<TBridge, THypRunBridge>>,
    layout: &mut DockingLayout
) -> Vec<PassivateStateChange<'b, TBridge>>
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

fn internal_ui<'a, 'b, TBridge: Bridge, THypRunBridge: HypRunBridge>(
    ui: &mut egui::Ui,
    view: &mut PassivateView<TBridge, THypRunBridge>,
    context: &mut PassivateViewContext<'a, 'b, TBridge>
)
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
                if let Some(selected_hyp_id) = &state.selected_hyp
                {
                    let hyp = state.hyp_session.hyps().entry(selected_hyp_id.chain()).or_none();

                    if let Some(hyp) = hyp
                    {
                        let hyp_details = HypDetails {
                            hyp,
                            snapshot_handles: view_state.snapshot_handles()
                        };

                        details_view.ui(ui, Some(&hyp_details));
                    }
                }
                else
                {
                    details_view.ui(ui, None);
                }

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
