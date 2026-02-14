use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::passivate_state::PassivateState;
use passivate_core::passivate_state_change::PassivateStateChange;
use passivate_delegation::tx_rx::Rx;
use passivate_egui_core::passivate_view_state::PassivateViewState;
use passivate_egui_docking::dock_views::DockViews;
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_views::passivate_views::{PassivateView, PassivateViews};
use passivate_egui_views::{passivate_layout, passivate_ui};
use passivate_log::log_message::LogMessage;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_run_bridge::RunHypsBridge;
use passivate_model_bridge::hyp_session_event::HypSessionEvent;
use passivate_model_bridge::source_change_event::SourceChangeEvent;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_change::HypSessionChange;

pub struct AppState<TBridge: Bridge>
{
    session: HypSession<TBridge>,
    state: PassivateState<TBridge>,
    view_state: PassivateViewState<TBridge>,
    dock_views: DockViews<PassivateView>,
    configuration: ConfigurationManager,
    first_update: bool
}

impl<TBridge: Bridge> AppState<TBridge>
{
    pub fn new(
        session: HypSession<TBridge>,
        state: PassivateState<TBridge>,
        view_state: PassivateViewState<TBridge>,
        dock_views: DockViews<PassivateView>,
        configuration: ConfigurationManager
    ) -> Self
    {
        Self {
            session,
            state,
            view_state,
            dock_views,
            configuration,
            first_update: true
        }
    }

    pub fn update_app(
        &mut self,
        egui_context: &egui::Context,
        layout: &mut DockingLayout,
        run_hyps: &impl RunHypsBridge<TBridge>,
        source_change_rx: &impl Rx<SourceChangeEvent>,
        session_event_rx: &impl Rx<HypSessionEvent<TBridge>>,
        log_rx: &impl Rx<LogMessage>
    )
    {
        let mut rerun_required = self.first_update;
        self.first_update = false;

        if source_change_rx.try_recv().is_ok()
        {
            rerun_required = true;
        }

        let session_change = self.session.update_next(session_event_rx).and_then(map_session_change);

        {
            self.state.update_state(session_change.as_ref());

            let configuration = &*self.configuration.acquire();
            self.view_state
                .update_view_state(session_change.as_ref(), configuration, egui_context, log_rx);
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
            rerun_required |= ui_change.requires_rerun();

            match ui_change
            {
                PassivateStateChange::ConfigurationChanged(configuration_change) =>
                {
                    _ = self.configuration.change(configuration_change);
                }
                ui_change =>
                {
                    self.state.update_state(Some(&ui_change));

                    let configuration = &*self.configuration.acquire();
                    self.view_state
                        .update_view_state(Some(&ui_change), configuration, egui_context, log_rx);
                }
            }
        }

        if rerun_required
        {
            let configuration = &*self.configuration.acquire();

            run_hyps.run_all(configuration.clone(), self.configuration.paths().clone());
        }
    }
}

fn map_session_change<TBridge: Bridge>(change: HypSessionChange<TBridge>) -> Option<PassivateStateChange<TBridge>>
{
    match change
    {
        HypSessionChange::HypUpdated(single_hyp) => Some(PassivateStateChange::HypDetailsChanged(single_hyp)),
        HypSessionChange::NewHyp(_) => None
    }
}

#[cfg(feature = "testing")]
#[bon::bon]
impl<TBridge: Bridge> AppState<TBridge>
{
    #[builder]
    pub fn stub(
        #[builder(default = HypSession::new())] session: HypSession<TBridge>,
        #[builder(default = true)] first_update: bool
    ) -> (AppState<TBridge>, DockingLayout)
    {
        let state = PassivateState::new();
        let view_state = PassivateViewState::default();
        let configuration = ConfigurationManager::stub().call();

        let views = PassivateViews::stub();

        let layout = passivate_layout::default(&views);
        let dock_views = DockViews::new(views.into());
        let app_state = AppState {
            session,
            state,
            view_state,
            dock_views,
            configuration,
            first_update
        };

        (app_state, layout)
    }
}

#[cfg(test)]
pub mod tests
{
    use std::path::PathBuf;

    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{Key, Queryable};
    use itertools::Itertools;
    use maybe_owned::MaybeOwned;
    use mockall::predicate::{always, eq};
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_egui_docking::view::View;
    use passivate_egui_views::passivate_views::PassivateViews;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_log::log_message::LogMessage;
    use passivate_model_bridge::hyp_report::HypReport;
    use passivate_model_bridge::hyp_run_bridge::MockRunHypsBridge;
    use passivate_model_bridge::hyp_session_bridge::{CompleteRunBridge, SendHypBridge, StartRunBridge};
    use passivate_model_bridge::hyp_state::HypState;
    use passivate_model_bridge::source_change_event::SourceChangeEvent;
    use passivate_run_rust::model::{RustBridge, RustHyp};
    use passivate_testing::model::{TestHyp, TestHypKind, TestSession};

    use crate::app_state::AppState;
    use crate::testing::app_state::UpdateApp;

    #[test]
    pub fn selecting_a_test_shows_it_in_details_view()
    {
        let mut session = TestSession::new();
        session.start_run();
        session.send_hyp(HypReport::new_fixed(
            TestHypKind::Hyp(TestHyp::new("example_test")),
            HypState::Passed
        ));
        session.complete_run();

        let (mut app_state, mut layout) = AppState::stub().session(session.into()).call();

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
        let (mut session_tx, session_rx) = crossbeam_channel::unbounded();
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().call();

        let views = PassivateViews::stub();
        let session_tab = layout.dock_state().find_tab(&views.session_dock().id()).unwrap();
        layout.dock_state().set_active_tab(session_tab);

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                .with_session_rx(MaybeOwned::Borrowed(&session_rx))
                .call();
        });

        let hyp_info = example_hyp();
        let hyp_report = HypReport::new_fixed(hyp_info, HypState::Passed);

        session_tx.start_run();
        ui.step();
        session_tx.send_hyp(hyp_report);
        ui.step();
        session_tx.complete_run();
        ui.step();

        let test_entry = ui.get_by_label("example_test");
        test_entry.click();
        ui.step();

        ui.step();
        ui.snapshot(&test_name!());
    }

    #[test]
    pub fn hyps_are_run_upon_first_update()
    {
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().call();
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().once().return_const(());
        mock_run_hyps.expect_run_single().never();

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                .with_run_hyps(MaybeOwned::Borrowed(&mock_run_hyps))
                .call();
        });

        ui.step();
    }

    #[test]
    pub fn hyps_are_run_upon_source_change_event()
    {
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().first_update(false).call();
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().once().return_const(());
        mock_run_hyps.expect_run_single().never();
        let (source_change_tx, source_change_rx) = crossbeam_channel::unbounded();

        source_change_tx.send(SourceChangeEvent::File(PathBuf::default())).unwrap();

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                .with_run_hyps(MaybeOwned::Borrowed(&mock_run_hyps))
                .with_source_change_rx(MaybeOwned::Borrowed(&source_change_rx))
                .call();
        });

        ui.step();
    }

    #[test]
    pub fn when_configuration_view_enables_coverage_hyps_run_with_coverage_enabled()
    {
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().first_update(false).call();
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().once().with(
            eq(PassivateConfiguration {
                coverage_enabled: true,
                ..Default::default()
            }),
            always()
        );
        mock_run_hyps.expect_run_single().never();

        let views = PassivateViews::stub();
        let configuration_tab = layout.dock_state().find_tab(&views.configuration_dock().id()).unwrap();
        layout.dock_state().set_active_tab(configuration_tab);

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
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().call();

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
    pub fn log_is_stored_in_view_state()
    {
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().call();
        let (log_tx, log_rx) = crossbeam_channel::unbounded();

        log_tx.send(LogMessage::new("example log")).unwrap();

        {
            // TODO: Make this possible to run without needing the UI parts?
            let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
                UpdateApp::with(&mut app_state, ui.ctx(), &mut layout)
                    .with_log_rx(MaybeOwned::Borrowed(&log_rx))
                    .call();
            });

            ui.run();
        }

        assert_eq!(1, app_state.view_state.logs().len());
    }

    #[test]
    pub fn configuring_snapshots_path_starts_a_hyp_run()
    {
        let (mut app_state, mut layout) = AppState::<RustBridge>::stub().first_update(false).call();
        let mut mock_run_hyps = MockRunHypsBridge::new();
        mock_run_hyps.expect_run_all().return_const(()).once();

        let views = PassivateViews::stub();
        let configuration_tab = layout.dock_state().find_tab(&views.configuration_dock().id()).unwrap();
        layout.dock_state().set_active_tab(configuration_tab);

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

    fn example_hyp() -> RustHyp
    {
        RustHyp::new_single(HypId::new("example_package", "example_crate", "example_test"))
    }
}
