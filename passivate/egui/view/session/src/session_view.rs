use egui::{Color32, RichText, Ui};
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_core::hyp::Hyp;
use passivate_model_core::hyp_session::HypSession;
use passivate_model_core::hyp_session_state_error::HypSessionStateError;

use crate::specialize_session_ui::SpecializeSessionUi;

pub struct SessionView;

pub(crate) enum HypUiAction
{
    Selected
}

impl SessionView
{
    pub fn ui<'a, TBridge: Bridge>(&mut self, ui: &mut Ui, session: &'a HypSession<TBridge>) -> Option<&'a Hyp<TBridge>>
    {
        match session.activity()
        {
            Ok(state) => self.show_session_state(ui, &state),
            Err(error) => self.show_error_state(ui, error)
        }

        let mut selected_hyp = None;

        for hyp in session.hyps().iter()
        {
            match hyp.ui(ui)
            {
                Some(HypUiAction::Selected) => selected_hyp = Some(hyp),
                None => ()
            }
        }

        selected_hyp
    }

    fn show_session_state(&mut self, ui: &mut Ui, state: &HypState)
    {
        let text = match state
        {
            HypState::Unknown => RichText::new("Idle").size(16.0).color(Color32::GREEN),
            HypState::Running => RichText::new("Running").size(16.0).color(Color32::GREEN),
            HypState::Failed => RichText::new("Failed").size(16.0).color(Color32::RED),
            HypState::Passed => RichText::new("Passed").size(16.0).color(Color32::GREEN)
        };

        ui.label(text);
    }

    fn show_error_state<TBridge: Bridge>(&mut self, ui: &mut Ui, error: &HypSessionStateError<TBridge>)
    {
        let text = RichText::new(error.to_string()).size(32.0).color(Color32::RED);

        ui.label(text);
    }
}

pub(crate) fn hyp_button(ui: &mut Ui, text: impl Into<String>, color: Color32) -> Option<HypUiAction>
{
    let text = RichText::new(text).size(16.0).color(color);

    if ui.button(text).clicked()
    {
        return Some(HypUiAction::Selected);
    }

    None
}

pub(crate) fn hyp_label(ui: &mut Ui, text: impl Into<String>)
{
    let text = RichText::new(text).size(16.0).color(Color32::GRAY);

    ui.label(text);
}

pub(crate) fn show_hyp<'a, TBridge: Bridge>(ui: &mut Ui, hyp: &'a Hyp<TBridge>, text: impl Into<String>)
-> Option<HypUiAction>
{
    match hyp.state()
    {
        HypState::Failed => hyp_button(ui, text, Color32::RED),
        HypState::Passed => hyp_button(ui, text, Color32::GREEN),
        HypState::Unknown =>
        {
            hyp_label(ui, text);
            None
        }
        HypState::Running => hyp_button(ui, text, Color32::LIGHT_BLUE)
    }
}

#[cfg(test)]
mod tests
{
    use egui_kittest::Harness;
    use passivate_hyp_names::test_name;
    use passivate_model_bridge::hyp_session_event::HypSessionEvent;
    use passivate_model_core::hyp_session::HypSession;
    use passivate_model_rust::RustBridge;

    use crate::session_view::SessionView;

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
}
