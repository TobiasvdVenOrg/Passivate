use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_egui_docking::dock_views::DockViews;
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_views::passivate_view_state::PassivateViewState;
use passivate_egui_views::passivate_views::PassivateView;

pub struct AppState
{
    state: PassivateState,
    view_state: PassivateViewState,
    dock_views: DockViews<PassivateView>,
    configuration: ConfigurationManager
}

impl AppState
{
    pub fn new(
        state: PassivateState,
        view_state: PassivateViewState,
        dock_views: DockViews<PassivateView>,
        configuration: ConfigurationManager
    ) -> Self
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
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_core::passivate_state::PassivateState;
    use passivate_delegation::{Rx, Tx};
    use passivate_egui_docking::dock_views::DockViews;
    use passivate_egui_docking::docking_layout::DockingLayout;
    use passivate_egui_views::passivate_layout;
    use passivate_egui_views::passivate_view_state::PassivateViewState;
    use passivate_egui_views::passivate_views::PassivateViews;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_model_session::hyp::Hyp;
    use passivate_model_session::hyp_session::HypSession;
    use passivate_model_session::hyp_session_event::HypSessionEvent;
    use passivate_model_session::hyp_state::HypState;
    use passivate_testing::path_resolution::test_data_path;

    use crate::app_state::AppState;

    #[test]
    pub fn selecting_a_test_shows_it_in_details_view()
    {
        let (mut app_state, mut layout) = example_app_state(Rx::stub());

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            app_state.update_and_ui(ui.ctx(), &mut layout);
        });

        ui.step();
        let test_entry = ui.get_by_label("example_test");
        test_entry.click();

        ui.step();
        ui.step();
        ui.snapshot(&test_name!());
    }

    #[test]
    pub fn when_a_test_is_selected_and_then_changes_status_the_details_view_also_updates()
    {
        let (hyp_run_tx, hyp_run_rx) = Tx::new();

        let (mut app_state, mut layout) = example_app_state(hyp_run_rx);

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            app_state.update_and_ui(ui.ctx(), &mut layout);
        });

        ui.step();

        let test_entry = ui.get_by_label("example_test");
        test_entry.click();
        ui.step();

        let example_hyp = example_hyp(HypState::Passed);
        hyp_run_tx.send(HypSessionEvent::HypCompleted(example_hyp.id));

        ui.step();
        ui.snapshot(&test_name!());
    }

    fn example_app_state(hyp_run_rx: Rx<HypSessionEvent>) -> (AppState, DockingLayout)
    {
        let example_hyp = example_hyp(HypState::Failed);

        todo!("Add the hyp to the state");

        let mut session = HypSession::new();
        let passivate_state = PassivateState::with_initial_session_state(session, hyp_run_rx);
        let view_state = PassivateViewState::default();
        let configuration = ConfigurationManager::new(
            PassivateConfiguration {
                snapshot_directories: vec![get_example_snapshots_path()],
                ..PassivateConfiguration::default()
            },
            Tx::stub()
        );

        let views = PassivateViews::stub();

        let layout = passivate_layout::default(&views);
        let dock_views = DockViews::new(views.into());
        let app_state = AppState::new(passivate_state, view_state, dock_views, configuration);

        (app_state, layout)
    }

    fn get_example_snapshots_path() -> Utf8PathBuf
    {
        test_data_path().join("example_snapshots")
    }

    fn example_hyp(state: HypState) -> Hyp
    {
        let hyp_id = HypId::new("example_package", "example_crate", "example_test");
        Hyp::new(hyp_id, state)
    }
}
