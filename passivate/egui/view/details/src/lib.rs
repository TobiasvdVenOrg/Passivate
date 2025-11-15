use egui::{Color32, RichText, TextureHandle, Ui};
use passivate_delegation::Tx;
use passivate_egui_core::HypDetails;
use passivate_egui_hyp_snapshots::SnapshotError;
use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
use passivate_model_core::hyp_run_trigger::HypRunTrigger;
use passivate_model_core::hyp_state::HypState;

pub struct DetailsView
{
    change_events: Tx<HypRunTrigger>
}

impl DetailsView
{
    pub fn new(change_events: Tx<HypRunTrigger>) -> Self
    {
        Self { change_events }
    }

    pub fn ui(&mut self, ui: &mut Ui, details: Option<&HypDetails>)
    {
        if let Some(details) = &details
        {
            let color = match details.hyp.current_state()
            {
                HypState::Passed => Color32::GREEN,
                HypState::Failed => Color32::RED,
                HypState::Unknown => Color32::GRAY
            };

            ui.horizontal(|ui| {
                let text = RichText::new(&details.hyp.name).size(16.0).color(color);
                ui.heading(text);

                if ui.button("Pin").clicked()
                {
                    self.change_events.send(HypRunTrigger::PinHyp {
                        id: details.hyp.id.clone()
                    });
                }

                if ui.button("Unpin").clicked()
                {
                    self.change_events.send(HypRunTrigger::ClearPinnedHyps);
                }
            });

            if !details.hyp.output.is_empty()
            {
                ui.add_space(16.0);

                for output in &details.hyp.output
                {
                    let output_line = RichText::new(output).size(12.0).color(color);
                    ui.label(output_line);
                }
            }

            if let Some(snapshot_handles) = &details.snapshot_handles
            {
                self.draw_snapshots(ui, snapshot_handles);
            }
        }
        else
        {
            ui.heading("No test selected");
        }
    }

    fn draw_snapshots(&self, ui: &mut Ui, snapshot_handles: &SnapshotHandles)
    {
        if let Some(current) = &snapshot_handles.current
        {
            if snapshot_handles.are_identical
            {
                Self::draw_snapshot(ui, current);
                return;
            }

            if snapshot_handles.new.is_some()
            {
                ui.heading("Current");
            }

            Self::draw_snapshot(ui, current);
        }

        if let Some(new) = &snapshot_handles.new
        {
            ui.horizontal(|ui| {
                ui.heading("New");

                let approve = RichText::new("Approve").size(12.0).color(Color32::GREEN);
                if ui.button(approve).clicked()
                {
                    self.change_events.send(HypRunTrigger::Hyp {
                        id: snapshot_handles.hyp_id.clone(),
                        update_snapshots: true
                    });
                }
            });

            Self::draw_snapshot(ui, new);
        }
    }

    fn draw_snapshot(ui: &mut Ui, snapshot: &Result<TextureHandle, SnapshotError>)
    {
        match &snapshot
        {
            Ok(snapshot) =>
            {
                ui.image(snapshot);
            }
            Err(error) =>
            {
                let text = RichText::new(error.to_string()).size(16.0).color(Color32::RED);
                ui.heading(text);
            }
        };
    }
}

#[cfg(test)]
mod tests
{
    use camino::Utf8PathBuf;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use galvanic_assert::matchers::*;
    use galvanic_assert::*;
    use passivate_delegation::Tx;
    use passivate_egui_hyp_snapshots::Snapshots;
    use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_model_core::hyp::Hyp;
    use passivate_model_core::hyp_run_trigger::HypRunTrigger;
    use passivate_model_core::hyp_state::HypState;
    use passivate_testing::path_resolution::test_data_path;
    use rstest::*;

    use crate::{DetailsView, HypDetails};

    #[test]
    pub fn show_a_passing_test()
    {
        let failing_test = example_hyp("example_crate::example_test", HypState::Passed);

        show_test(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_a_failing_test()
    {
        let failing_test = example_hyp("example_crate::example_test", HypState::Failed);

        show_test(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_a_failing_test_with_output()
    {
        let mut failing_test = Hyp::new(
            HypId::new("example_package", "example_crate", "example_test"),
            HypState::Failed
        );

        failing_test.add_output(String::from("this is some error output"));
        failing_test.add_output(String::from("you messed up"));

        show_test(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_snapshot_associated_with_test_rgb()
    {
        let test_with_snapshot = example_hyp("tests::example_snapshot_rgb", HypState::Failed);

        show_test(&test_name!(), test_with_snapshot);
    }

    #[test]
    pub fn show_snapshot_associated_with_test_rgba()
    {
        let test_with_snapshot = example_hyp("tests::example_snapshot_rgba", HypState::Failed);

        show_test(&test_name!(), test_with_snapshot);
    }

    #[test]
    pub fn show_current_and_new_snapshots_associated_with_test()
    {
        let test_with_changed_snapshot = example_hyp("tests::example_snapshot_changed", HypState::Failed);

        show_test(&test_name!(), test_with_changed_snapshot);
    }

    #[test]
    pub fn show_only_new_snapshot_associated_with_test_when_there_is_no_current_snapshot()
    {
        let test_first_run = example_hyp("tests::example_snapshot_only_new", HypState::Failed);

        show_test(&test_name!(), test_first_run);
    }

    #[test]
    pub fn show_only_one_snapshot_when_both_current_and_new_are_present_but_identical()
    {
        let test_run_identical_snapshot = example_hyp("tests::example_snapshot_identical", HypState::Failed);

        show_test(&test_name!(), test_run_identical_snapshot);
    }

    #[rstest]
    #[case::current_and_new("tests::example_snapshot_changed")]
    #[case::only_new("tests::example_snapshot_only_new")]
    pub fn approving_new_snapshot_emits_event_to_run_test_with_update_snapshots_enabled(#[case] hyp: &str)
    {
        let snapshot_test = example_hyp(hyp, HypState::Failed);

        let (test_run_tx, test_run_rx) = Tx::new();

        let mut details_view = DetailsView::new(test_run_tx);

        let mut details = None;

        let ui = |ui: &mut egui::Ui| {
            if details.is_none()
            {
                let snapshot = Snapshots::new(vec![get_example_snapshots_path()]).from_hyp(&snapshot_test.id);
                let snapshot_handles = SnapshotHandles::new(snapshot_test.id.clone(), snapshot, ui.ctx());
                details = Some(HypDetails::new(snapshot_test.clone(), Some(snapshot_handles)));
            }

            details_view.ui(ui, details.as_ref());
        };

        let mut harness = Harness::new_ui(ui);

        harness.run();

        let approve = harness.get_by_label("Approve");
        approve.click();
        harness.run();

        let approval_run = test_run_rx.drain().last().unwrap().clone();

        assert_that!(
            &approval_run,
            has_structure!(HypRunTrigger::Hyp {
                id: eq(HypId::new("example_package", "example_crate", hyp)),
                update_snapshots: eq(true)
            })
        );
    }

    fn show_test(test_name: &str, single_test: Hyp)
    {
        let mut details_view = DetailsView::new(Tx::stub());

        let ui = |ui: &mut egui::Ui| {
            let snapshot = Snapshots::new(vec![get_example_snapshots_path()]).from_hyp(&single_test.id);
            let snapshot_handles = SnapshotHandles::new(single_test.id.clone(), snapshot, ui.ctx());
            let details = HypDetails::new(single_test.clone(), Some(snapshot_handles));

            details_view.ui(ui, Some(&details));
        };

        let mut harness = Harness::new_ui(ui);

        harness.run();
        harness.fit_contents();
        harness.snapshot(test_name);
    }

    fn get_example_snapshots_path() -> Utf8PathBuf
    {
        test_data_path().join("example_snapshots")
    }

    fn example_hyp(name: &str, status: HypState) -> Hyp
    {
        let id = HypId::new("example_package", "example_crate", name);
        Hyp::new(id, status)
    }
}
