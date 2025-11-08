use egui::{Color32, RichText};
use passivate_hyp_model::hyp::Hyp;
use passivate_hyp_model::hyp_session::HypSession;
use passivate_hyp_model::hyp_state::HypState;

use crate::docking::docking_layout::DockId;
use crate::docking::view::View;

pub struct TestRunView;

impl View for TestRunView
{
    fn id(&self) -> DockId
    {
        DockId::from("hyp_run_view")
    }

    fn title(&self) -> String
    {
        String::from("Tests")
    }
}

impl TestRunView
{
    pub fn ui<'a>(&mut self, ui: &mut egui_dock::egui::Ui, session: &'a HypSession) -> Option<&'a Hyp>
    {
        match &session.state()
        {
            Ok(_) => todo!(),
            Err(_) => todo!()
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

    fn hyp_button<'a>(&self, ui: &mut egui_dock::egui::Ui, hyp: &'a Hyp, color: Color32) -> Option<&'a Hyp>
    {
        let text = RichText::new(&hyp.name).size(16.0).color(color);

        if ui.button(text).clicked()
        {
            return Some(hyp);
        }

        None
    }

    fn hyp_label(&self, ui: &mut egui_dock::egui::Ui, hyp: &Hyp)
    {
        let text = RichText::new(&hyp.name).size(16.0).color(Color32::GRAY);

        ui.label(text);
    }

    fn show_hyp<'a>(&self, ui: &mut egui_dock::egui::Ui, hyp: &'a Hyp) -> Option<&'a Hyp>
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
    use passivate_hyp_model::hyp::Hyp;
    use passivate_hyp_model::hyp_run::HypRun;
    use passivate_hyp_model::hyp_session::HypSession;
    use passivate_hyp_model::hyp_session_change::HypSessionEvent;
    use passivate_hyp_model::hyp_session_state::HypSessionState;
    use passivate_hyp_model::hyp_state::HypState;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;

    use crate::test_run_view::TestRunView;

    #[test]
    pub fn show_when_first_test_run_is_starting()
    {
        run_and_snapshot(session_from_state(HypSessionState::FirstRun), &test_name!());
    }

    #[test]
    pub fn show_when_no_tests_were_found()
    {
        run_and_snapshot(session_from_state(HypSessionState::Idle), &test_name!());
    }

    #[test]
    pub fn show_when_build_failed()
    {
        let build_failed = session_from_state(HypSessionState::BuildFailed("Something didn't compile!".to_string()));

        run_and_snapshot(build_failed, &test_name!());
    }

    #[test]
    pub fn show_tests_with_unknown_status_greyed_out()
    {
        let mut hyp_run = HypRun::default();
        hyp_run.add_hyp(example_hyp("example_test", HypState::Unknown));

        let session = HypSession::new(HypRun::default(), hyp_run);
        run_and_snapshot(session, &test_name!());
    }

    #[test]
    pub fn show_build_status_above_tests_while_compiling()
    {
        let mut hyp_run = HypRun::default();
        hyp_run.add_hyp(example_hyp("example_test", HypState::Unknown));

        let mut session = HypSession::new(HypRun::default(), hyp_run);
        session.update(HypSessionEvent::Compiling(
            "The build is working on something right now!".to_string()
        ));

        run_and_snapshot(session, &test_name!());
    }

    fn session_from_state(state: HypSessionState) -> HypSession
    {
        let mut session = HypSession::default();
        session.state = state;

        session
    }

    fn run_and_snapshot(session: HypSession, snapshot_name: &str)
    {
        let mut test_run_view = TestRunView;

        let ui = move |ui: &mut egui::Ui| {
            _ = test_run_view.ui(ui, &session);
        };

        let mut harness = Harness::new_ui(ui);

        harness.run();
        harness.fit_contents();
        harness.snapshot(snapshot_name);
    }

    fn example_hyp(name: &str, status: HypState) -> Hyp
    {
        let id = HypId::new("example_crate", name).unwrap();
        Hyp::new(id, status, vec![])
    }
}
