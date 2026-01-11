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
use passivate_model_bridge::hyp_run_bridge::RunHypsBridge;
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
    configuration: ConfigurationManager
}

impl AppState
{
    pub fn new(
        session: HypSession<RustBridge>,
        state: PassivateState<RustBridge>,
        view_state: PassivateViewState<RustBridge>,
        dock_views: DockViews<PassivateView>,
        configuration: ConfigurationManager
    ) -> Self
    {
        Self {
            session,
            state,
            view_state,
            dock_views,
            configuration
        }
    }

    pub fn update_app(
        &mut self,
        egui_context: &egui::Context,
        layout: &mut DockingLayout,
        run_hyps: &impl RunHypsBridge<RustBridge>,
        session_event_rx: &impl Rx<HypSessionEvent<RustBridge>>,
        log_rx: &impl Rx<LogMessage>
    )
    {
        let configuration = &*self.configuration.acquire();

        let session_change = self.session.update_next(session_event_rx).map(map_session_change).flatten();

        if let Some(session_change) = session_change
        {
            self.state.update_state(&session_change);
            self.view_state
                .update_view_state(&session_change, configuration, egui_context, log_rx);
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
                .update_view_state(&ui_change, configuration, egui_context, log_rx);
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
    use galvanic_assert::assert_that;
    use maybe_owned::MaybeOwned;
    use mockall::predicate::{always, eq};
    use mockall::{Predicate, predicate};
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_core::passivate_state::PassivateState;
    use passivate_delegation::{Rx, Tx};
    use passivate_egui_core::passivate_view_state::PassivateViewState;
    use passivate_egui_docking::dock_views::DockViews;
    use passivate_egui_docking::docking_layout::DockingLayout;
    use passivate_egui_view_configuration::ConfigurationView;
    use passivate_egui_views::passivate_layout;
    use passivate_egui_views::passivate_views::PassivateViews;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_model_bridge::hyp_run_bridge::MockRunHypsBridge;
    use passivate_model_bridge::hyp_run_request::HypRunOptions;
    use passivate_model_bridge::hyp_session_event::HypSessionEvent;
    use passivate_model_bridge::hyp_state::HypState;
    use passivate_model_core::hyp::Hyp;
    use passivate_model_core::hyp_session::HypSession;
    use passivate_model_rust::{RustBridge, RustHyp};
    use passivate_testing::path_resolution::test_data_path;

    use crate::app_state::AppState;
    use crate::testing::app_state::UpdateApp;

    #[test]
    pub fn selecting_a_test_shows_it_in_details_view()
    {
        let (mut app_state, mut layout) = example_app_state();

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            UpdateApp::with(&mut app_state, ui.ctx(), &mut layout).call();
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
        let (session_tx, session_rx) = crossbeam_channel::unbounded();
        let (mut app_state, mut layout) = example_app_state();

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                .with_session_rx(MaybeOwned::Borrowed(&session_rx))
                .call();
        });

        ui.step();

        let test_entry = ui.get_by_label("example_test");
        test_entry.click();
        ui.step();

        let example_hyp = example_hyp(HypState::Passed);
        session_tx.send(HypSessionEvent::HypCompleted(example_hyp.id().clone()));

        ui.step();
        ui.snapshot(&test_name!());
    }

    #[test]
    pub fn when_configuration_view_enables_coverage_hyps_run_with_coverage_enabled()
    {
        let (mut app_state, mut layout) = example_app_state();
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().once().with(
            eq(HypRunOptions {
                compute_coverage: true,
                ..Default::default()
            }),
            predicate::always()
        );
        mock_run_hyps.expect_run_single().never();

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                .with_run_hyps(MaybeOwned::Borrowed(&mock_run_hyps))
                .call();
        });

        let coverage_toggle = ui.get_by_label("Compute Coverage");
        coverage_toggle.click();

        ui.run();
    }

    #[test]
    pub fn enabling_coverage_in_coverage_view_modifies_configuration()
    {
        let configuration = ConfigurationManager::default();
        let test_run_handler = HypRunHandler::builder()
            .configuration(configuration.clone())
            .coverage(Box::new(MockComputeCoverage::new()))
            .coverage_tx(Tx::stub())
            .runner(HypRunner::faux())
            .hyp_run_tx(SessionEventTx::stub())
            .build();

        let mut coverage_view = CoverageView::new(Rx::stub(), configuration.clone());

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        let enable_button = harness.get_by_label("Enable");
        enable_button.click();

        harness.run();

        assert!(test_run_handler.coverage_enabled());

        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn configuring_snapshots_path_starts_a_hyp_run()
    {
        let configuration = ConfigurationManager::default();
        let (change_events_tx, change_events_rx) = Tx::new();
        let mut configuration_view = ConfigurationView::new(configuration.clone(), change_events_tx);

        let ui = |ui: &mut egui::Ui| {
            configuration_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        harness.get_by_role(Role::TextInput).type_text("Some/Path/");
        harness.run();

        // Simulate typing across multiple frames...
        harness.get_by_role(Role::TextInput).type_text("To/Snapshots");
        harness.get_by_role(Role::TextInput).press_keys(&[Key::Enter]);
        harness.run();

        drop(harness);

        assert_that!(
            &change_events_rx.drain().last().expect("expected change event").clone(),
            eq(HypRunTrigger::<RustBridge>::DefaultRun)
        );

        assert_eq!(
            configuration
                .get(|c| c.snapshot_directories.iter().exactly_one().unwrap().clone())
                .as_str(),
            "Some/Path/To/Snapshots"
        );
    }

    fn example_app_state() -> (AppState, DockingLayout)
    {
        let session = HypSession::new();
        let passivate_state = PassivateState::new();
        let view_state = PassivateViewState::default();
        let configuration = ConfigurationManager::new(PassivateConfiguration {
            snapshot_directories: vec![get_example_snapshots_path()],
            ..PassivateConfiguration::default()
        });

        let views = PassivateViews::stub();

        let layout = passivate_layout::default(&views);
        let dock_views = DockViews::new(views.into());
        let app_state = AppState::new(session, passivate_state, view_state, dock_views, configuration);

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
