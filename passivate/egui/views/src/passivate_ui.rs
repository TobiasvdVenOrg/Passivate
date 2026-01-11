use egui_dock::{DockArea, Style};
use passivate_configuration::configuration::PassivateConfiguration;
use passivate_core::passivate_state::PassivateState;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::{DockViewer, DockViews};
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_view_details::hyp_details::HypDetails;
use passivate_id_chain_tree::id_chain::IdChain;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_core::hyp_session::HypSession;

use crate::passivate_views::PassivateView;

/// This context object is used to grant access to broad state for views to use
/// Any state changes produced by views are aggregated in the context
/// These changes are returned to the caller after the view ui has finished
struct PassivateViewContext<'ctx, 'env, TBridge: Bridge>
{
    session: &'env HypSession<TBridge>,
    view_state: &'ctx PassivateViewState<TBridge>,
    state: &'ctx PassivateState<TBridge>,
    configuration: &'ctx PassivateConfiguration,
    changes: Vec<PassivateStateChange<'env, TBridge>>
}

impl<'ctx, 'env, TBridge: Bridge> PassivateViewContext<'ctx, 'env, TBridge>
{
    fn new(
        session: &'env HypSession<TBridge>,
        view_state: &'ctx PassivateViewState<TBridge>,
        state: &'ctx PassivateState<TBridge>,
        configuration: &'ctx PassivateConfiguration
    ) -> Self
    {
        Self {
            session,
            state,
            view_state,
            configuration,
            changes: Vec::new()
        }
    }
}

pub fn ui<'env, TBridge: Bridge>(
    session: &'env HypSession<TBridge>,
    view_state: &PassivateViewState<TBridge>,
    state: &PassivateState<TBridge>,
    configuration: &PassivateConfiguration,
    egui_context: &egui::Context,
    dock_views: &mut DockViews<PassivateView>,
    layout: &mut DockingLayout
) -> Vec<PassivateStateChange<'env, TBridge>>
{
    let mut passivate_context = PassivateViewContext::new(session, view_state, state, configuration);

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

    // egui_context.request_repaint();

    passivate_context.changes
}

fn internal_ui<'a, 'b, TBridge: Bridge>(
    ui: &mut egui::Ui,
    view: &mut PassivateView,
    context: &mut PassivateViewContext<'a, 'b, TBridge>
)
{
    let change = {
        let session = context.session;
        let state = context.state;
        let view_state = context.view_state;
        let configuration = context.configuration;

        match view
        {
            PassivateView::Configuration(configuration_view) =>
            {
                configuration_view.ui(ui, configuration);
                None
            }
            PassivateView::Coverage(coverage_view) =>
            {
                coverage_view
                    .ui(ui, &state.coverage)
                    .map(PassivateStateChange::ConfigurationChanged)
            }
            PassivateView::Details(details_view) =>
            {
                if let Some(selected_hyp_id) = &state.selected_hyp
                {
                    let hyp = session.hyps().entry(selected_hyp_id.chain()).or_none();

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
                    details_view.ui::<TBridge>(ui, None);
                }

                None
            }
            PassivateView::Log(log_view) =>
            {
                log_view.ui(ui, view_state.logs());
                None
            }
            PassivateView::HypRun(test_run_view) => test_run_view.ui(ui, session).map(PassivateStateChange::HypSelected)
        }
    };

    if let Some(change) = change
    {
        context.changes.push(change);
    }
}
