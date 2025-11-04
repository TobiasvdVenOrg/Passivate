use egui::{Color32, RichText};
use passivate_hyp_model::hyp_run::HypRun;
use passivate_hyp_model::hyp_session::HypSession;
use passivate_hyp_model::hyp_session_state::HypSessionState;
use passivate_hyp_model::single_hyp::SingleHyp;
use passivate_hyp_model::single_hyp_status::SingleHypStatus;

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
    pub fn ui<'a>(&mut self, ui: &mut egui_dock::egui::Ui, session: &'a HypSession) -> Option<&'a SingleHyp>
    {
        match &session.state
        {
            HypSessionState::FirstRun =>
            {
                ui.heading("Starting first test run...");
            }
            HypSessionState::Idle =>
            {
                if session.no_tests()
                {
                    ui.heading("No tests found.");
                }
            }
            HypSessionState::Building(message) =>
            {
                ui.heading("Building");

                let text = RichText::new(message).size(12.0).color(Color32::YELLOW);
                ui.label(text);
            }
            HypSessionState::Running =>
            {}
            HypSessionState::BuildFailed(build_failure) =>
            {
                ui.heading("Build failed.");

                let text = RichText::new(build_failure).size(16.0).color(Color32::RED);
                ui.label(text);
            }
            HypSessionState::Failed(run_tests_error_status) =>
            {
                ui.heading("Failed to run tests.");

                let text = RichText::new(run_tests_error_status).size(16.0).color(Color32::RED);
                ui.label(text);
            }
        }

        let mut selected_hyp = None;

        let hyp_run = session.current_run();

        for hyp in hyp_run.hyps()
        {
            if let Some(new_selection) = self.show_hyp(ui, hyp)
            {
                selected_hyp = Some(new_selection)
            }
        }

        selected_hyp
    }

    fn hyp_button<'a>(&self, ui: &mut egui_dock::egui::Ui, hyp: &'a SingleHyp, color: Color32) -> Option<&'a SingleHyp>
    {
        let text = RichText::new(&hyp.name).size(16.0).color(color);

        if ui.button(text).clicked()
        {
            return Some(hyp);
        }

        None
    }

    fn hyp_label(&self, ui: &mut egui_dock::egui::Ui, hyp: &SingleHyp)
    {
        let text = RichText::new(&hyp.name).size(16.0).color(Color32::GRAY);

        ui.label(text);
    }

    fn show_hyp<'a>(&self, ui: &mut egui_dock::egui::Ui, hyp: &'a SingleHyp) -> Option<&'a SingleHyp>
    {
        match hyp.status
        {
            SingleHypStatus::Failed => self.hyp_button(ui, hyp, Color32::RED),
            SingleHypStatus::Passed => self.hyp_button(ui, hyp, Color32::GREEN),
            SingleHypStatus::Unknown =>
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
    use passivate_hyp_model::hyp_run::HypRun;
    use passivate_hyp_model::hyp_run_events::HypRunEvent;
    use passivate_hyp_model::hyp_session::HypSession;
    use passivate_hyp_model::hyp_session_state::HypSessionState;
    use passivate_hyp_model::single_hyp::SingleHyp;
    use passivate_hyp_model::single_hyp_status::SingleHypStatus;
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
        hyp_run.add_hyp(example_hyp("example_test", SingleHypStatus::Unknown));

        let session = HypSession::new(HypRun::default(), hyp_run);
        run_and_snapshot(session, &test_name!());
    }

    #[test]
    pub fn show_build_status_above_tests_while_compiling()
    {
        let mut hyp_run = HypRun::default();
        hyp_run.add_hyp(example_hyp("example_test", SingleHypStatus::Unknown));

        let mut session = HypSession::new(HypRun::default(), hyp_run);
        session.update(HypRunEvent::Compiling(
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

    fn example_hyp(name: &str, status: SingleHypStatus) -> SingleHyp
    {
        let id = HypId::new("example_crate", name).unwrap();
        SingleHyp::new(id, status, vec![])
    }
}
