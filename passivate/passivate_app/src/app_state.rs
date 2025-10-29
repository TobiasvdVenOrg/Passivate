use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_egui::passivate_view::PassivateView;
use passivate_egui::passivate_view_state::{HypDetails, PassivateViewState};
use passivate_egui::snapshots::Snapshots;
use passivate_egui::snapshots::snapshot_handles::SnapshotHandles;
use passivate_hyp_model::hyp_run_events::HypRunChange;

pub struct AppState
{
    pub state: PassivateState,
    pub view_state: PassivateViewState,
    pub configuration: ConfigurationManager
}

impl AppState
{
    pub fn new(state: PassivateState, view_state: PassivateViewState, configuration: ConfigurationManager) -> Self
    {
        Self {
            state,
            view_state,
            configuration
        }
    }

    pub fn update(ui: &mut egui::Ui, view: &mut PassivateView, state: &mut AppState)
    {
        if let Some(change) = state.state.update()
        {
            match change
            {
                HypRunChange::HypDetailsChanged(single_hyp) => {
                    if let Some(details) = &mut state.view_state.hyp_details
                    && details.hyp.id == single_hyp.id
                    {
                        details.hyp = single_hyp.clone();
                    }
                },
            }
        }

        match view
        {
            PassivateView::Configuration(configuration_view) => configuration_view.ui(ui),
            PassivateView::Coverage(coverage_view) => coverage_view.ui(ui),
            PassivateView::Details(details_view) => details_view.ui(ui, state.view_state.hyp_details.as_ref()),
            PassivateView::Log(log_view) => log_view.ui(ui),
            PassivateView::HypRun(test_run_view) => 
            {
                if let Some(selected_hyp) = test_run_view.ui(ui, &state.state.hyp_run)
                {
                    state.state.selected_hyp = Some(selected_hyp.id.clone());

                    let snapshot_directories = state.configuration.get(|c| c.snapshot_directories.clone());

                    if !snapshot_directories.is_empty()
                    {
                        let snapshot = Snapshots::new(snapshot_directories).from_hyp(&selected_hyp.id);
                        let snapshot_handles = SnapshotHandles::new(selected_hyp.id.clone(), snapshot, ui.ctx());

                        state.view_state.hyp_details = Some(HypDetails::new(selected_hyp.clone(), Some(snapshot_handles)));
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
    use galvanic_assert::{assert_that, is_variant};
    use itertools::Itertools;
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_core::passivate_state::PassivateState;
    use passivate_delegation::{Rx, Tx};
    use passivate_egui::passivate_view::PassivateView;
    use passivate_egui::passivate_view_state::PassivateViewState;
    use passivate_egui::{DetailsView, TestRunView};
    use passivate_hyp_model::hyp_run_events::HypRunEvent;
    use passivate_hyp_model::single_hyp::SingleHyp;
    use passivate_hyp_model::single_hyp_status::SingleHypStatus;
    use passivate_hyp_model::hyp_run::HypRun;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_testing::path_resolution::test_data_path;

    use crate::app_state::AppState;

    #[test]
    pub fn selecting_a_test_shows_it_in_details_view()
    {
        let mut app_state = example_app_state(Rx::stub());

        {
            let mut test_run_view = PassivateView::HypRun(TestRunView);
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

    fn example_app_state(hyp_run_rx: Rx<HypRunEvent>) -> AppState {
        let mut hyp_run = HypRun::default();
        let example_hyp = example_hyp();
        hyp_run.hyps.insert(example_hyp.id.clone(), example_hyp);
        
        let passivate_state = PassivateState::with_initial_run_state(hyp_run, hyp_run_rx);
        let view_state = PassivateViewState::default();
        let configuration = ConfigurationManager::new(PassivateConfiguration {
            snapshot_directories: vec![get_example_snapshots_path()],
            ..PassivateConfiguration::default()
        }, Tx::stub());
        
        AppState::new(passivate_state, view_state, configuration)
    }
    
    #[test]
    pub fn when_a_test_is_selected_and_then_changes_status_the_details_view_also_updates()
    {
        let (hyp_run_tx, hyp_run_rx) = Tx::new();

        let mut app_state = example_app_state(hyp_run_rx);

        {
            let mut test_run_view = PassivateView::HypRun(TestRunView);
            let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui| {
                AppState::update(ui, &mut test_run_view, &mut app_state);
            });
            
            test_run_ui.run();
            let test_entry = test_run_ui.get_by_label("example_test");
            test_entry.click();
            test_run_ui.run();
        }

        {
            let mut details_view = PassivateView::Details(DetailsView::new(Tx::stub()));
            let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui| {
                AppState::update(ui, &mut details_view, &mut app_state);
            });

            details_ui.run();

            let mut example_hyp = example_hyp();
            example_hyp.status = SingleHypStatus::Passed;
            hyp_run_tx.send(HypRunEvent::TestFinished(example_hyp));

            details_ui.run();
            details_ui.fit_contents();
            details_ui.snapshot(&test_name!());
        }
        
        let hyp = app_state.state.hyp_run.hyps.values().exactly_one().unwrap();
        assert_that!(&hyp.status, is_variant!(SingleHypStatus::Passed));
    }

    fn get_example_snapshots_path() -> Utf8PathBuf
    {
        test_data_path().join("example_snapshots")
    }

    fn example_hyp() -> SingleHyp 
    {
        let hyp_id = HypId::new("example_crate", "example_test").unwrap();
        SingleHyp::new(hyp_id, SingleHypStatus::Failed, vec![])
    }
}
