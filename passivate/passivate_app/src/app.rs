use camino::Utf8PathBuf;
use eframe::Frame;
use egui::Context;
use egui_dock::{DockArea, Style};
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_delegation::Rx;
use passivate_egui::details_view::HypDetails;
use passivate_egui::passivate_view_state::PassivateViewState;
use passivate_egui::snapshots::snapshot_handles::SnapshotHandles;
use passivate_egui::snapshots::Snapshots;
use passivate_hyp_model::hyp_run_events::HypRunEvent;
use passivate_egui::docking::dock_views::{DockViewer, DockViews};
use passivate_egui::docking::layout_management::LayoutManagement;
use passivate_egui::passivate_view::PassivateView;

pub struct App<'a>
{
    layout: LayoutManagement,
    dock_views: DockViews<PassivateView>,
    state: &'a mut PassivateState,
    view_state: PassivateViewState<'a>,
    hyp_run_rx: Rx<HypRunEvent>,
    configuration: ConfigurationManager
}

impl<'a> App<'a>
{
    pub fn new(
        layout: LayoutManagement,
        dock_views: DockViews<PassivateView>,
        state: &'a mut PassivateState,
        view_state: PassivateViewState<'a>,
        hyp_run_rx: Rx<HypRunEvent>,
        configuration: ConfigurationManager
    ) -> Self
    {
        Self {
            layout,
            dock_views,
            state,
            view_state,
            hyp_run_rx,
            configuration
        }
    }

    fn main_update(&mut self)
    {
        if let Ok(hyp_run_event) = self.hyp_run_rx.try_recv()
        {
            self.state.persisted.hyp_run.update(hyp_run_event);
        }
    }

    fn custom_ui(ui: &mut egui::Ui, view: &mut PassivateView, state: &mut App<'_>)
    {
        match view
        {
            PassivateView::Configuration(configuration_view) => configuration_view.ui(ui),
            PassivateView::Coverage(coverage_view) => coverage_view.ui(ui),
            PassivateView::Details(details_view) => details_view.ui(ui, state.view_state.hyp_details.as_ref()),
            PassivateView::Log(log_view) => log_view.ui(ui),
            PassivateView::TestRun(test_run_view) => 
            {
                if let Some(selected_id) = test_run_view.ui(ui, &state.state.persisted.hyp_run)
                {
                    if let Some(snapshots_path) = state.configuration.get(|c| c.snapshots_path.clone())
                    {
                        state.state.persisted.selected_hyp = Some(selected_id.clone());


                        let hyp = state.state.persisted.hyp_run.tests.find(&selected_id).expect("huh?");
                        let snapshot = Snapshots::new(snapshots_path).from_hyp(&selected_id);
                        let snapshot_handles = SnapshotHandles::new(selected_id.clone(), snapshot, ui.ctx());
                        state.view_state.hyp_details = Some(HypDetails::new(hyp, Some(snapshot_handles)));
                    }
                }
            }
        }
    }
}

impl eframe::App for App<'_>
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        self.main_update();

        let layout = self.layout.get_current();

        let mut dock_viewer = DockViewer {
            dock_views: &mut self.dock_views,
            state: self,
            custom_ui: Self::custom_ui
        };

        DockArea::new(layout.get_state())
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, &mut dock_viewer);

        ctx.request_repaint();
    }
}
