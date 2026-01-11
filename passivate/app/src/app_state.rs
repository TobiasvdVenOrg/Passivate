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
        let session_change = self.session.update_next(session_event_rx).map(map_session_change).flatten();

        match session_change
        {
            Some(PassivateStateChange::ConfigurationChanged(configuration_change)) =>
            {
                _ = self.configuration.change(configuration_change);
            }
            Some(session_change) =>
            {
                self.state.update_state(&session_change);

                let configuration = &*self.configuration.acquire();
                self.view_state
                    .update_view_state(&session_change, configuration, egui_context, log_rx);
            }
            None =>
            {}
        }

        let ui_changes = {
            let configuration = &*self.configuration.acquire();

            passivate_ui::ui(
                &self.session,
                &self.view_state,
                &self.state,
                configuration,
                egui_context,
                &mut self.dock_views,
                layout
            )
        };

        for ui_change in ui_changes
        {
            match ui_change
            {
                PassivateStateChange::ConfigurationChanged(configuration_change) =>
                {
                    _ = self.configuration.change(configuration_change);
                }
                ui_change =>
                {
                    self.state.update_state(&ui_change);

                    let configuration = &*self.configuration.acquire();
                    self.view_state
                        .update_view_state(&ui_change, configuration, egui_context, log_rx);
                }
            }
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
    use std::fmt::Debug;

    use camino::Utf8PathBuf;
    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{Key, Queryable};
    use itertools::Itertools;
    use maybe_owned::MaybeOwned;
    use mockall::predicate;
    use mockall::predicate::eq;
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_core::passivate_state::PassivateState;
    use passivate_egui_core::passivate_view_state::PassivateViewState;
    use passivate_egui_docking::dock_views::DockViews;
    use passivate_egui_docking::docking_layout::DockingLayout;
    use passivate_egui_docking::view::View;
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

    struct DebugHarness<'a, State = ()>(&'a Harness<'a, State>);

    impl<'a, State> Debug for DebugHarness<'a, State>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            f.debug_struct("State").field("tree", &self.0.kittest_state().root()).finish()
        }
    }

    #[test]
    pub fn enabling_coverage_in_coverage_view_modifies_configuration()
    {
        let (mut app_state, mut layout) = example_app_state();

        let views = PassivateViews::stub();
        let coverage_tab = layout.dock_state().find_tab(&views.coverage_dock().id()).unwrap();
        layout.dock_state().set_active_tab(coverage_tab);

        {
            let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
                UpdateApp::with(&mut app_state, ui.ctx(), &mut layout).call();
            });

            ui.run();
            let coverage_toggle = ui.get_by_label("Enable");
            coverage_toggle.click();
            ui.run();
        }

        let coverage_enabled = app_state.configuration.get(|c| c.coverage_enabled);
        assert!(coverage_enabled);
    }

    #[test]
    pub fn configuring_snapshots_path_starts_a_hyp_run()
    {
        let (mut app_state, mut layout) = example_app_state();
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().once();

        {
            let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
                UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                    .with_run_hyps(MaybeOwned::Borrowed(&mock_run_hyps))
                    .call();
            });

            ui.get_by_role(Role::TextInput).type_text("Some/Path/");
            ui.run();

            // Simulate typing across multiple frames...
            ui.get_by_role(Role::TextInput).type_text("To/Snapshots");
            ui.get_by_role(Role::TextInput).press_keys(&[Key::Enter]);
            ui.run();
        }

        assert_eq!(
            app_state
                .configuration
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
