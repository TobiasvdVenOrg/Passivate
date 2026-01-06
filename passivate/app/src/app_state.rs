use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_delegation::Rx;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::DockViews;
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_views::passivate_ui;
use passivate_egui_views::passivate_views::PassivateView;
use passivate_log::log_message::LogMessage;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_change::HypSessionChange;
use passivate_model_rust::RustBridge;

pub struct AppState
{
    session: HypSession<RustBridge>,
    state: PassivateState<RustBridge>,
    view_state: PassivateViewState<RustBridge>,
    dock_views: DockViews<PassivateView>,
    configuration: ConfigurationManager,
    session_event_rx: crossbeam_channel::Receiver<HypSessionEvent<RustBridge>>,
    log_rx: crossbeam_channel::Receiver<LogMessage>
}

impl AppState
{
    pub fn new(
        session: HypSession<RustBridge>,
        state: PassivateState<RustBridge>,
        view_state: PassivateViewState<RustBridge>,
        dock_views: DockViews<PassivateView>,
        configuration: ConfigurationManager,
        session_event_rx: crossbeam_channel::Receiver<HypSessionEvent<RustBridge>>,
        log_rx: crossbeam_channel::Receiver<LogMessage>
    ) -> Self
    {
        Self {
            session,
            state,
            view_state,
            dock_views,
            configuration,
            session_event_rx,
            log_rx
        }
    }

    pub fn update_app(&mut self, egui_context: &egui::Context, layout: &mut DockingLayout)
    {
        let configuration = &*self.configuration.acquire();

        let session_change = self
            .session
            .update_next(&self.session_event_rx)
            .map(map_session_change)
            .flatten();

        if let Some(session_change) = session_change
        {
            self.state.update_state(&session_change);
            self.view_state
                .update_view_state(&session_change, configuration, egui_context, &self.log_rx);
        }

        let ui_changes = passivate_ui::ui(
            &self.session,
            &self.view_state,
            &self.state,
            configuration,
            egui_context,
            &mut self.dock_views,
            layout
        );

        for ui_change in ui_changes
        {
            self.state.update_state(&ui_change);
            self.view_state
                .update_view_state(&ui_change, configuration, egui_context, &self.log_rx);
        }
    }
}

fn map_session_change(change: HypSessionChange<RustBridge>) -> Option<PassivateStateChange<RustBridge>>
{
    match change
    {
        HypSessionChange::HypUpdated(single_hyp) => Some(PassivateStateChange::HypDetailsChanged(single_hyp)),
        HypSessionChange::NewHyp(_) => None
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
    use passivate_egui_core::passivate_view_state::PassivateViewState;
    use passivate_egui_docking::dock_views::DockViews;
    use passivate_egui_docking::docking_layout::DockingLayout;
    use passivate_egui_views::passivate_layout;
    use passivate_egui_views::passivate_views::PassivateViews;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_model_bridge::hyp_run_bridge::MockHypRunBridge;
    use passivate_model_bridge::hyp_state::HypState;
    use passivate_model_core::hyp::Hyp;
    use passivate_model_core::hyp_session::HypSession;
    use passivate_model_core::hyp_session_event::HypSessionEvent;
    use passivate_model_rust::{RustBridge, RustHyp};
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
        hyp_run_tx.send(HypSessionEvent::HypCompleted(example_hyp.id().clone()));

        ui.step();
        ui.snapshot(&test_name!());
    }

    fn example_app_state(hyp_run_rx: Rx<HypSessionEvent<RustBridge>>) -> (AppState<MockHypRunBridge>, DockingLayout)
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

        let views = PassivateViews::<RustBridge, MockHypRunBridge>::stub();

        let layout = passivate_layout::default(&views);
        let dock_views = DockViews::new(views.into());
        let app_state = AppState::new(passivate_state, view_state, dock_views, configuration);

        (app_state, layout)
    }

    fn get_example_snapshots_path() -> Utf8PathBuf
    {
        test_data_path().join("example_snapshots")
    }

    fn example_hyp(state: HypState) -> Hyp<RustBridge>
    {
        let hyp_id = RustHyp::new_single(HypId::new("example_package", "example_crate", "example_test"));
        Hyp::with_state(hyp_id, state)
    }
}
