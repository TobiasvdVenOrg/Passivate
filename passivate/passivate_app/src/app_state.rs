use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_egui::docking::dock_views::DockViews;
use passivate_egui::docking::docking_layout::DockingLayout;
use passivate_egui::passivate_view::PassivateView;
use passivate_egui::passivate_view_state::PassivateViewState;

pub struct AppState
{
    state: PassivateState,
    view_state: PassivateViewState,
    dock_views: DockViews<PassivateView>,
    configuration: ConfigurationManager
}

impl AppState
{
    pub fn new(state: PassivateState, view_state: PassivateViewState, dock_views: DockViews<PassivateView>, configuration: ConfigurationManager) -> Self
    {
        Self {
            state,
            view_state,
            dock_views,
            configuration
        }
    }

    pub fn update(&mut self, egui_context: &egui::Context)
    {
        let change = self.state.update();

        if let Some(change) = &change
        {
            self.view_state.update(change, &self.configuration, egui_context);
        }
    }

    pub fn update_and_ui(&mut self, egui_context: &egui::Context, layout: &mut DockingLayout)
    {
        self.update(egui_context);

        let changes = self.view_state.ui(&self.state, egui_context, &mut self.dock_views, layout);

        for change in &changes
        {
            self.view_state.update(change, &self.configuration, egui_context);
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
    use passivate_egui::{DetailsView, TestRunView, passivate_layout};
    use passivate_hyp_model::hyp_run::HypRun;
    use passivate_hyp_model::hyp_run_events::HypRunEvent;
    use passivate_hyp_model::single_hyp::SingleHyp;
    use passivate_hyp_model::single_hyp_status::SingleHypStatus;
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
                app_state.update(ui.ctx());
                app_state.ui(ui.ctx(), passivate_layout::default(app_state), dock_views);
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

    fn example_app_state(hyp_run_rx: Rx<HypRunEvent>) -> AppState
    {
        let mut hyp_run = HypRun::default();
        let example_hyp = example_hyp();
        hyp_run.hyps.insert(example_hyp.id.clone(), example_hyp);

        let passivate_state = PassivateState::with_initial_run_state(hyp_run, hyp_run_rx);
        let view_state = PassivateViewState::default();
        let configuration = ConfigurationManager::new(
            PassivateConfiguration {
                snapshot_directories: vec![get_example_snapshots_path()],
                ..PassivateConfiguration::default()
            },
            Tx::stub()
        );

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
