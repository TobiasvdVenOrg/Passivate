use egui::{Color32, RichText};
use passivate_delegation::{Rx, Tx};
use passivate_hyp_model::{single_test::SingleTest, single_test_status::SingleTestStatus, test_run::{TestRun, TestRunState}};
use passivate_hyp_names::hyp_id::HypId;

use crate::docking::{docking_layout::DockId, view::View};

pub struct TestRunView
{
    receiver: Rx<TestRun>,
    test_details: Tx<Option<SingleTest>>,
    status: TestRun,
    selected_hyp: Option<HypId>
}

impl TestRunView
{
    pub fn new(receiver: Rx<TestRun>, test_details: Tx<Option<SingleTest>>) -> TestRunView
    {
        TestRunView {
            receiver,
            test_details,
            status: TestRun::default(),
            selected_hyp: None
        }
    }

    fn test_button(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest, color: Color32) -> Option<SingleTest>
    {
        let text = RichText::new(&test.name).size(16.0).color(color);

        if ui.button(text).clicked()
        {
            return Some(test.clone());
        }

        None
    }

    fn test_label(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest)
    {
        let text = RichText::new(&test.name).size(16.0).color(Color32::GRAY);

        ui.label(text);
    }

    fn send_selected_hyp_details(&mut self)
    {
        if let Some(selected_hyp) = &self.selected_hyp
        {
            self.test_details.send(self.status.tests.find(selected_hyp));
        }
    }

    fn show_test(&self, ui: &mut egui_dock::egui::Ui, test: &SingleTest) -> Option<SingleTest>
    {
        match test.status
        {
            SingleTestStatus::Failed => self.test_button(ui, test, Color32::RED),
            SingleTestStatus::Passed => self.test_button(ui, test, Color32::GREEN),
            SingleTestStatus::Unknown =>
            {
                self.test_label(ui, test);
                None
            }
        }
    }
}

impl View for TestRunView
{
    fn id(&self) -> DockId
    {
        "test_run_view".into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        if let Ok(status) = self.receiver.try_recv()
        {
            self.status = status;
            self.send_selected_hyp_details();
        }

        match &self.status.state
        {
            TestRunState::FirstRun =>
            {
                ui.heading("Starting first test run...");
            }
            TestRunState::Idle =>
            {
                if self.status.tests.is_empty()
                {
                    ui.heading("No tests found.");
                }
            }
            TestRunState::Building(message) =>
            {
                ui.heading("Building");

                let text = RichText::new(message).size(12.0).color(Color32::YELLOW);
                ui.label(text);
            }
            TestRunState::Running =>
            {}
            TestRunState::BuildFailed(build_failure) =>
            {
                ui.heading("Build failed.");

                let text = RichText::new(&build_failure.message).size(16.0).color(Color32::RED);
                ui.label(text);
            }
            TestRunState::Failed(run_tests_error_status) =>
            {
                ui.heading("Failed to run tests.");

                let text = RichText::new(&run_tests_error_status.inner_error_display).size(16.0).color(Color32::RED);
                ui.label(text);
            }
        }

        let mut send = false;

        for test in &self.status.tests
        {
            if let Some(new_selection) = self.show_test(ui, test)
            {
                self.selected_hyp = Some(new_selection.id);
                send = true;
            }
        }

        if send
        {
            self.send_selected_hyp_details();
        }
    }

    fn title(&self) -> String
    {
        "Tests Status".to_string()
    }
}

#[cfg(test)]
mod tests
{
    use egui_kittest::Harness;
    use passivate_delegation::Tx;
    use passivate_hyp_names::{hyp_id::HypId, test_name};
    use passivate_hyp_model::{single_test::SingleTest, single_test_status::SingleTestStatus, test_run::{BuildFailedTestRun, TestRun, TestRunState}, test_run_events::TestRunEvent};

    use crate::{docking::view::View, test_run_view::TestRunView};

    #[test]
    pub fn show_when_first_test_run_is_starting()
    {
        run_and_snapshot(TestRun::from_state(TestRunState::FirstRun), &test_name!());
    }

    #[test]
    pub fn show_when_no_tests_were_found()
    {
        run_and_snapshot(TestRun::from_state(TestRunState::Idle), &test_name!());
    }

    #[test]
    pub fn show_when_build_failed()
    {
        let build_failed = TestRun::from_state(TestRunState::BuildFailed(BuildFailedTestRun {
            message: "Something didn't compile!".to_string()
        }));

        run_and_snapshot(build_failed, &test_name!());
    }

    #[test]
    pub fn show_tests_with_unknown_status_greyed_out()
    {
        let mut active = TestRun::default();
        active.tests.add(example_hyp("example_test", SingleTestStatus::Unknown));

        run_and_snapshot(active, &test_name!());
    }

    #[test]
    pub fn show_build_status_above_tests_while_compiling()
    {
        let mut active = TestRun::default();
        active.tests.add(example_hyp("example_test", SingleTestStatus::Unknown));
        active.update(TestRunEvent::Compiling("The build is working on something right now!".to_string()));

        run_and_snapshot(active, &test_name!());
    }

    fn run_and_snapshot(tests_status: TestRun, snapshot_name: &str)
    {
        let (tx, rx) = Tx::new();
        let mut tests_status_view = TestRunView::new(rx, Tx::stub());

        let ui = |ui: &mut egui::Ui| {
            tests_status_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        tx.send(tests_status);

        harness.run();
        harness.fit_contents();
        harness.snapshot(snapshot_name);
    }

    fn example_hyp(name: &str, status: SingleTestStatus) -> SingleTest
    {
        let id = HypId::new("example_crate", name).unwrap();
        SingleTest::new(id, status, vec![])
    }
}
