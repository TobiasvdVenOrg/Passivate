use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_egui::{details_view::HypDetails, passivate_view::PassivateView, passivate_view_state::PassivateViewState, snapshots::{snapshot_handles::SnapshotHandles, Snapshots}};

pub struct AppState<'a>
{
    pub state: &'a mut PassivateState,
    pub view_state: PassivateViewState,
    pub configuration: ConfigurationManager
}

impl<'a> AppState<'a>
{
    pub fn new(state: &'a mut PassivateState, view_state: PassivateViewState, configuration: ConfigurationManager) -> Self
    {
        Self {
            state,
            view_state,
            configuration
        }
    }

    pub fn update(ui: &mut egui::Ui, view: &mut PassivateView, state: &mut AppState<'_>)
    {
        state.state.update();

        match view
        {
            PassivateView::Configuration(configuration_view) => configuration_view.ui(ui),
            PassivateView::Coverage(coverage_view) => coverage_view.ui(ui),
            PassivateView::Details(details_view) => details_view.ui(ui, state.view_state.hyp_details.as_ref()),
            PassivateView::Log(log_view) => log_view.ui(ui),
            PassivateView::TestRun(test_run_view) => 
            {
                if let Some(selected_id) = test_run_view.ui(ui, &state.state.persisted.hyp_run)
                    && let Some(snapshots_path) = state.configuration.get(|c| c.snapshots_path.clone())
                    {
                        state.state.persisted.selected_hyp = Some(selected_id.clone());

                        let hyp = state.state.persisted.hyp_run.tests.find(&selected_id).expect("huh?");
                        let snapshot = Snapshots::new(snapshots_path).from_hyp(&selected_id);
                        let snapshot_handles = SnapshotHandles::new(selected_id.clone(), snapshot, ui.ctx());

                        state.view_state.hyp_details = Some(HypDetails::new(hyp.clone(), Some(snapshot_handles)));
                    }
            }
        }
    }
}
