use egui::{Color32, RichText};
use passivate_hyp_model::{hyp_run_state::HypRunState, single_hyp::SingleHyp, single_hyp_status::SingleHypStatus, test_run::TestRun};
use passivate_hyp_names::hyp_id::HypId;

pub struct TestRunView;

impl TestRunView
{
    pub fn ui(&mut self, ui: &mut egui_dock::egui::Ui, test_run: &TestRun) -> Option<HypId>
    {
        match &test_run.state
        {
            HypRunState::FirstRun =>
            {
                ui.heading("Starting first test run...");
            }
            HypRunState::Idle =>
            {
                if test_run.tests.is_empty()
                {
                    ui.heading("No tests found.");
                }
            }
            HypRunState::Building(message) =>
            {
                ui.heading("Building");

                let text = RichText::new(message).size(12.0).color(Color32::YELLOW);
                ui.label(text);
            }
            HypRunState::Running =>
            {}
            HypRunState::BuildFailed(build_failure) =>
            {
                ui.heading("Build failed.");

                let text = RichText::new(&build_failure.message).size(16.0).color(Color32::RED);
                ui.label(text);
            }
            HypRunState::Failed(run_tests_error_status) =>
            {
                ui.heading("Failed to run tests.");

                let text = RichText::new(&run_tests_error_status.inner_error_display).size(16.0).color(Color32::RED);
                ui.label(text);
            }
        }

        let mut selected_hyp = None;

        for test in &test_run.tests
        {
            if let Some(new_selection) = self.show_test(ui, test)
            {
                selected_hyp = Some(new_selection.id)
            }
        }

        selected_hyp
    }

    fn test_button(&self, ui: &mut egui_dock::egui::Ui, test: &SingleHyp, color: Color32) -> Option<SingleHyp>
    {
        let text = RichText::new(&test.name).size(16.0).color(color);

        if ui.button(text).clicked()
        {
            return Some(test.clone());
        }

        None
    }

    fn test_label(&self, ui: &mut egui_dock::egui::Ui, test: &SingleHyp)
    {
        let text = RichText::new(&test.name).size(16.0).color(Color32::GRAY);

        ui.label(text);
    }

    fn show_test(&self, ui: &mut egui_dock::egui::Ui, test: &SingleHyp) -> Option<SingleHyp>
    {
        match test.status
        {
            SingleHypStatus::Failed => self.test_button(ui, test, Color32::RED),
            SingleHypStatus::Passed => self.test_button(ui, test, Color32::GREEN),
            SingleHypStatus::Unknown =>
            {
                self.test_label(ui, test);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use egui_kittest::Harness;
    use passivate_hyp_names::{hyp_id::HypId, test_name};
    use passivate_hyp_model::{hyp_run_events::HypRunEvent, hyp_run_state::HypRunState, single_hyp::SingleHyp, single_hyp_status::SingleHypStatus, test_run::{BuildFailedTestRun, TestRun}};

    use crate::test_run_view::TestRunView;

    #[test]
    pub fn show_when_first_test_run_is_starting()
    {
        run_and_snapshot(TestRun::from_state(HypRunState::FirstRun), &test_name!());
    }

    #[test]
    pub fn show_when_no_tests_were_found()
    {
        run_and_snapshot(TestRun::from_state(HypRunState::Idle), &test_name!());
    }

    #[test]
    pub fn show_when_build_failed()
    {
        let build_failed = TestRun::from_state(HypRunState::BuildFailed(BuildFailedTestRun {
            message: "Something didn't compile!".to_string()
        }));

        run_and_snapshot(build_failed, &test_name!());
    }

    #[test]
    pub fn show_tests_with_unknown_status_greyed_out()
    {
        let mut active = TestRun::default();
        active.tests.add(example_hyp("example_test", SingleHypStatus::Unknown));

        run_and_snapshot(active, &test_name!());
    }

    #[test]
    pub fn show_build_status_above_tests_while_compiling()
    {
        let mut active = TestRun::default();
        active.tests.add(example_hyp("example_test", SingleHypStatus::Unknown));
        active.update(HypRunEvent::Compiling("The build is working on something right now!".to_string()));

        run_and_snapshot(active, &test_name!());
    }

    fn run_and_snapshot(hyp_run: TestRun, snapshot_name: &str)
    {
        let mut test_run_view = TestRunView;

        let ui = move |ui: &mut egui::Ui| {
            _ = test_run_view.ui(ui, &hyp_run);
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
