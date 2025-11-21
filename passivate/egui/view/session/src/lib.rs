#![feature(min_specialization)]

mod rust_specialization;
mod specialize_session_ui;

use egui::{Color32, RichText, Ui};
use passivate_model_core::bridge::Bridge;
use passivate_model_core::hyp::Hyp;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_state::{HypSessionState, HypSessionStateError};
use passivate_model_core::hyp_state::HypState;

use crate::specialize_session_ui::SpecializeSessionUi;

pub struct SessionView;

impl SessionView
{
    pub fn ui<'a, TBridge: Bridge>(&mut self, ui: &mut Ui, session: &'a HypSession<TBridge>) -> Option<&'a Hyp>
    {
        match &session.state()
        {
            Ok(state) => self.show_session_state(ui, state),
            Err(error) => self.show_error_state(ui, *error)
        }

        let projects = session.projects();

        for project in projects
        {
            project.ui(ui);
        }

        let mut selected_hyp = None;

        for hyp in session.all_hyps()
        {
            if let Some(new_selection) = self.show_hyp(ui, hyp)
            {
                selected_hyp = Some(new_selection)
            }
        }

        selected_hyp
    }

    fn show_session_state(&mut self, ui: &mut Ui, state: &HypSessionState)
    {
        let text = match state
        {
            HypSessionState::Idle => RichText::new("Idle").size(16.0).color(Color32::GREEN),
            HypSessionState::Starting => RichText::new("Starting").size(16.0).color(Color32::GREEN),
            HypSessionState::Compiling => RichText::new("Compiling").size(16.0).color(Color32::GREEN),
            HypSessionState::Running => RichText::new("Running").size(16.0).color(Color32::GREEN)
        };

        ui.label(text);
    }

    fn show_error_state<TBridge: Bridge>(&mut self, ui: &mut Ui, error: &HypSessionStateError<TBridge>)
    {
        let text = RichText::new(error.to_string()).size(32.0).color(Color32::RED);

        ui.label(text);
    }

    fn hyp_button<'a>(&self, ui: &mut Ui, hyp: &'a Hyp, color: Color32) -> Option<&'a Hyp>
    {
        let text = RichText::new(&hyp.name).size(16.0).color(color);

        if ui.button(text).clicked()
        {
            return Some(hyp);
        }

        None
    }

    fn hyp_label(&self, ui: &mut Ui, hyp: &Hyp)
    {
        let text = RichText::new(&hyp.name).size(16.0).color(Color32::GRAY);

        ui.label(text);
    }

    fn show_hyp<'a>(&self, ui: &mut Ui, hyp: &'a Hyp) -> Option<&'a Hyp>
    {
        match hyp.current_state()
        {
            HypState::Failed => self.hyp_button(ui, hyp, Color32::RED),
            HypState::Passed => self.hyp_button(ui, hyp, Color32::GREEN),
            HypState::Unknown =>
            {
                self.hyp_label(ui, hyp);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use egui_kittest::Harness;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_model_core::hyp::Hyp;
    use passivate_model_core::hyp_session::HypSession;
    use passivate_model_core::hyp_session_event::HypSessionEvent;
    use passivate_model_core::hyp_state::HypState;
    use passivate_model_rust::RustBridge;

    use crate::SessionView;

    #[test]
    pub fn show_when_session_is_in_error_state()
    {
        let mut session = HypSession::new();

        // Start the session twice to trigger an error
        session.update_all([HypSessionEvent::RunStarted, HypSessionEvent::RunStarted]);

        run_and_snapshot(session, test_name!());
    }

    #[test]
    pub fn show_when_hyp_run_has_started()
    {
        let mut session = HypSession::new();

        session.update(HypSessionEvent::RunStarted);

        run_and_snapshot(session, test_name!());
    }

    #[test]
    pub fn show_when_no_tests_were_found()
    {
        todo!();
    }

    #[test]
    pub fn show_when_build_failed()
    {
        todo!();
    }

    #[test]
    pub fn show_tests_with_unknown_status_greyed_out()
    {
        todo!();
    }

    #[test]
    pub fn show_build_status_above_tests_while_compiling()
    {
        todo!();
    }

    fn run_and_snapshot(session: HypSession<RustBridge>, snapshot_name: impl Into<String>)
    {
        let mut test_run_view = SessionView;

        let ui = move |ui: &mut egui::Ui| {
            _ = test_run_view.ui(ui, &session);
        };

        let mut harness = Harness::new_ui(ui);

        harness.run();
        harness.fit_contents();
        harness.snapshot(&snapshot_name.into());
    }

    fn example_hyp(name: &str, status: HypState) -> Hyp
    {
        let id = HypId::new("example_package", "example_crate", name);
        Hyp::new(id, status)
    }
}
