use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_egui::details_view::HypDetails;
use passivate_egui::passivate_view::PassivateView;
use passivate_egui::passivate_view_state::PassivateViewState;
use passivate_egui::snapshots::Snapshots;
use passivate_egui::snapshots::snapshot_handles::SnapshotHandles;

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
                {
                    let snapshot_directories = state.configuration.get(|c| c.snapshot_directories.clone());

                    if !snapshot_directories.is_empty()
                    {
                        state.state.persisted.selected_hyp = Some(selected_id.clone());

                        let hyp = state.state.persisted.hyp_run.tests.find(&selected_id).expect("huh?");
                        let snapshot = Snapshots::new(snapshot_directories).from_hyp(&selected_id);
                        let snapshot_handles = SnapshotHandles::new(selected_id.clone(), snapshot, ui.ctx());

                        state.view_state.hyp_details = Some(HypDetails::new(hyp.clone(), Some(snapshot_handles)));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests
{
    use camino::Utf8PathBuf;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_core::passivate_state::{PassivateState, PersistedPassivateState};
    use passivate_delegation::{Rx, Tx};
    use passivate_egui::passivate_view::PassivateView;
    use passivate_egui::passivate_view_state::PassivateViewState;
    use passivate_egui::{DetailsView, TestRunView};
    use passivate_hyp_model::single_test::SingleTest;
    use passivate_hyp_model::single_test_status::SingleTestStatus;
    use passivate_hyp_model::test_run::TestRun;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_testing::path_resolution::test_data_path;

    use crate::app_state::AppState;

    #[test]
    pub fn selecting_a_test_shows_it_in_details_view()
    {
        let mut hyp_run = TestRun::default();
        let hyp_id = HypId::new("example_crate", "example_test").unwrap();
        let example_hyp = SingleTest::new(hyp_id, SingleTestStatus::Failed, vec![]);
        hyp_run.tests.add(example_hyp);
        let hyp_run_state = PersistedPassivateState {
            hyp_run,
            selected_hyp: None
        };

        let mut passivate_state = PassivateState::with_persisted_state(hyp_run_state, Rx::stub());
        let view_state = PassivateViewState::default();
        let configuration = ConfigurationManager::new(PassivateConfiguration {
            snapshot_directories: vec![get_example_snapshots_path()],
            ..PassivateConfiguration::default()
        }, Tx::stub());

        let mut app_state = AppState::new(&mut passivate_state, view_state, configuration);

        {
            let mut test_run_view = PassivateView::TestRun(TestRunView);
            let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui| {
                AppState::update(ui, &mut test_run_view, &mut app_state);
            });
            
            test_run_ui.run();
            let test_entry = test_run_ui.get_by_label("example_test");
            test_entry.click();
            test_run_ui.run();
        }

        assert!(app_state.view_state.hyp_details.is_some());

        let mut details_view = PassivateView::Details(DetailsView::new(Tx::stub()));
        let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui| {
            AppState::update(ui, &mut details_view, &mut app_state);
        });

        details_ui.run();
        details_ui.fit_contents();
        details_ui.snapshot(&test_name!());
    }

    fn get_example_snapshots_path() -> Utf8PathBuf
    {
        test_data_path().join("example_snapshots")
    }
}
