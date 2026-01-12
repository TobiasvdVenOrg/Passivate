use egui::{Color32, RichText, TextureHandle, Ui};
use passivate_egui_hyp_snapshots::snapshot_error::SnapshotError;
use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_state::HypState;

use crate::hyp_details::HypDetails;

pub struct DetailsView;

impl DetailsView
{
    pub fn ui<TBridge: Bridge>(&mut self, ui: &mut Ui, details: Option<&HypDetails<'_, TBridge>>)
    {
        if let Some(details) = details
        {
            let color = match details.hyp.state()
            {
                HypState::Passed => Color32::GREEN,
                HypState::Failed => Color32::RED,
                HypState::Unknown => Color32::GRAY,
                HypState::Running => Color32::LIGHT_BLUE
            };

            ui.horizontal(|ui| {
                let text = RichText::new(details.hyp.name()).size(16.0).color(color);
                ui.heading(text);

                if ui.button("Pin").clicked()
                {
                    todo!();
                }

                if ui.button("Unpin").clicked()
                {
                    todo!();
                }
            });

            if details.hyp.has_output()
            {
                ui.add_space(16.0);

                for output in details.hyp.iter_output()
                {
                    let output_line = RichText::new(output.to_string()).size(12.0).color(color);
                    ui.label(output_line);
                }
            }

            if let Some(snapshot_handles) = &details.snapshot_handles
            {
                self.draw_snapshots::<TBridge>(ui, snapshot_handles);
            }
        }
        else
        {
            ui.heading("No test selected");
        }
    }

    fn draw_snapshots<TBridge: Bridge>(&self, ui: &mut Ui, snapshot_handles: &SnapshotHandles<TBridge::Id>)
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
                    todo!();
                }
            });

            Self::draw_snapshot(ui, new);
        }
    }

    fn draw_snapshot(ui: &mut Ui, snapshot: &Result<TextureHandle, SnapshotError>)
    {
        match snapshot
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
    use passivate_egui_hyp_snapshots::Snapshots;
    use passivate_egui_hyp_snapshots::snapshot_handles::SnapshotHandles;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_model_bridge::bridge::Bridge;
    use passivate_model_bridge::hyp_state::HypState;
    use passivate_model_core::hyp::Hyp;
    use passivate_run_rust::model::{RustBridge, RustHyp};
    use passivate_testing::model::{TestHyp, TestHypKind, TestOutput, TestSession};
    use passivate_testing::path_resolution::test_data_path;
    use rstest::*;

    use crate::details_view::DetailsView;
    use crate::hyp_details::HypDetails;

    #[test]
    pub fn show_a_passing_test()
    {
        let failing_test = example_hyp("example_crate::example_test", HypState::Passed);

        show_hyp(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_a_failing_test()
    {
        let failing_test = example_hyp("example_crate::example_test", HypState::Failed);

        show_hyp(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_a_failing_test_with_output()
    {
        let mut failing_test: Hyp<TestSession> = Hyp::with_state(
            TestHypKind::Hyp(TestHyp::new("example_package::example_crate::example_test")),
            HypState::Failed
        );

        failing_test.add_output(TestOutput::from("this is some error output"));
        failing_test.add_output(TestOutput::from("you messed up"));

        show_hyp(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_snapshot_associated_with_test_rgb()
    {
        let test_with_snapshot = example_hyp("tests::example_snapshot_rgb", HypState::Failed);

        show_hyp(&test_name!(), test_with_snapshot);
    }

    #[test]
    pub fn show_snapshot_associated_with_test_rgba()
    {
        let test_with_snapshot = example_hyp("tests::example_snapshot_rgba", HypState::Failed);

        show_hyp(&test_name!(), test_with_snapshot);
    }

    #[test]
    pub fn show_current_and_new_snapshots_associated_with_test()
    {
        let test_with_changed_snapshot = example_hyp("tests::example_snapshot_changed", HypState::Failed);

        show_hyp(&test_name!(), test_with_changed_snapshot);
    }

    #[test]
    pub fn show_only_new_snapshot_associated_with_test_when_there_is_no_current_snapshot()
    {
        let test_first_run = example_hyp("tests::example_snapshot_only_new", HypState::Failed);

        show_hyp(&test_name!(), test_first_run);
    }

    #[test]
    pub fn show_only_one_snapshot_when_both_current_and_new_are_present_but_identical()
    {
        let test_run_identical_snapshot = example_hyp("tests::example_snapshot_identical", HypState::Failed);

        show_hyp(&test_name!(), test_run_identical_snapshot);
    }

    #[rstest]
    #[case::current_and_new("tests::example_snapshot_changed")]
    #[case::only_new("tests::example_snapshot_only_new")]
    pub fn approving_new_snapshot_emits_event_to_run_test_with_update_snapshots_enabled(#[case] _hyp: &str)
    {
        todo!();
    }

    fn show_hyp<TBridge: Bridge>(test_name: &str, hyp: Hyp<TBridge>)
    {
        let mut details_view = DetailsView;

        let ui = |ui: &mut egui::Ui| {
            let snapshot = Snapshots::new(vec![get_example_snapshots_path()]).from_hyp(&hyp);
            let snapshot_handles = SnapshotHandles::new(hyp.id().clone(), snapshot, ui.ctx());
            let details = HypDetails {
                hyp: &hyp,
                snapshot_handles: Some(&snapshot_handles)
            };

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

    fn example_hyp(name: &str, status: HypState) -> Hyp<RustBridge>
    {
        let id = RustHyp::new_single(HypId::new("example_package", "example_crate", name));
        Hyp::with_state(id, status)
    }
}
