use egui::{Color32, RichText, TextureHandle};
use passivate_delegation::Tx;
use passivate_hyp_model::snapshots::snapshot_handles::SnapshotHandles;
use passivate_hyp_model::snapshots::SnapshotError;
use passivate_hyp_model::{change_event::ChangeEvent, single_test::SelectedHyp};
use passivate_hyp_model::single_test_status::SingleTestStatus;

pub struct DetailsView
{
    change_events: Tx<ChangeEvent>
}

impl DetailsView
{
    pub fn new(
        change_events: Tx<ChangeEvent>
    ) -> Self
    {
        Self {
            change_events
        }
    }

    pub fn ui(&mut self, ui: &mut egui_dock::egui::Ui, details: &Option<SelectedHyp>)
    {
        if let Some(details) = &details
        {
            let color = match details.hyp.status
            {
                SingleTestStatus::Passed => Color32::GREEN,
                SingleTestStatus::Failed => Color32::RED,
                SingleTestStatus::Unknown => Color32::GRAY
            };

            ui.horizontal(|ui| {
                let text = RichText::new(&details.hyp.name).size(16.0).color(color);
                ui.heading(text);

                if ui.button("Pin").clicked()
                {
                    self.change_events.send(ChangeEvent::PinHyp {
                        id: details.hyp.id.clone()
                    });
                }

                if ui.button("Unpin").clicked()
                {
                    self.change_events.send(ChangeEvent::ClearPinnedHyps);
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
    }

    fn draw_snapshots(&mut self, ui: &mut egui_dock::egui::Ui, snapshot_handles: &SnapshotHandles)
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
                    self.change_events.send(ChangeEvent::SingleHyp {
                        id: snapshot_handles.hyp_id.clone(),
                        update_snapshots: true
                    });
                }
            });

            Self::draw_snapshot(ui, new);
        }
    }

    fn draw_snapshot(ui: &mut egui_dock::egui::Ui, snapshot: &Result<TextureHandle, SnapshotError>)
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
    use passivate_hyp_model::change_event::ChangeEvent;
    use passivate_hyp_model::hyp_run_events::HypRunEvent;
    use passivate_hyp_model::passivate_state::PassivateState;
    use passivate_hyp_model::single_test::{SelectedHyp, SingleTest};
    use passivate_hyp_model::single_test_status::SingleTestStatus;
    use passivate_hyp_model::test_run::TestRun;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_testing::path_resolution::test_data_path;
    use rstest::*;

    use crate::details_view::DetailsView;
    use crate::test_run_view::TestRunView;

    #[test]
    pub fn show_a_passing_test()
    {
        let failing_test = example_hyp("example_crate::example_test", SingleTestStatus::Passed);

        show_test(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_a_failing_test()
    {
        let failing_test = example_hyp("example_crate::example_test", SingleTestStatus::Failed);

        show_test(&test_name!(), failing_test);
    }

    #[test]
    pub fn show_a_failing_test_with_output()
    {
        let failing_test = SingleTest::new(
            HypId::new("example_crate", "example_test").unwrap(),
            SingleTestStatus::Failed,
            vec!["this is some error output".to_string(), "you messed up".to_string()]
        );

        show_test(&test_name!(), failing_test);
    }

    #[test]
    pub fn selecting_a_test_shows_it_in_details_view()
    {
        let mut details_view = DetailsView::new(Tx::stub());

        let mut passivate_state = PassivateState {
            hyp_run: TestRun::default(),
            selected_hyp: None
        };

        passivate_state.hyp_run
            .tests
            .add(example_hyp("tests::example_test", SingleTestStatus::Failed));

        let mut test_run_view = TestRunView;

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            test_run_view.ui(ui, &passivate_state.hyp_run, &mut passivate_state.selected_hyp);
            details_view.ui(ui, &passivate_state.selected_hyp);
        });

        ui.run();

        let test_entry = ui.get_by_label("example_test");
        test_entry.click();

        ui.run();
        ui.fit_contents();
        ui.snapshot(&test_name!());
    }

    #[test]
    pub fn when_a_test_is_selected_and_then_changes_status_the_details_view_also_updates()
    {
        let (tx, rx) = Tx::new();

        let mut details_view = DetailsView::new(Tx::stub());
        let mut test_run_view = TestRunView;

        let mut state = PassivateState {
            hyp_run: TestRun::default(),
            selected_hyp: None
        };

        let mut ui = Harness::new_ui(|ui: &mut egui::Ui| {
            if let Ok(event) = rx.try_recv()
            {
                state.hyp_run.update(event);
            }
            
            test_run_view.ui(ui, &state.hyp_run, &mut state.selected_hyp);
            details_view.ui(ui, &state.selected_hyp);
        });

        tx.send(HypRunEvent::TestFinished(example_hyp(
            "tests::example_test",
            SingleTestStatus::Failed
        )));

        ui.run();

        let test_entry = ui.get_by_label("example_test");
        test_entry.click();

        ui.run();

        tx.send(HypRunEvent::TestFinished(example_hyp(
            "tests::example_test",
            SingleTestStatus::Passed
        )));

        ui.run();
        ui.fit_contents();
        ui.snapshot(&test_name!());
    }

    #[test]
    pub fn show_snapshot_associated_with_test_rgb()
    {
        let test_with_snapshot = example_hyp("tests::example_snapshot_rgb", SingleTestStatus::Failed);

        show_test(&test_name!(), test_with_snapshot);
    }

    #[test]
    pub fn show_snapshot_associated_with_test_rgba()
    {
        let test_with_snapshot = example_hyp("tests::example_snapshot_rgba", SingleTestStatus::Failed);

        show_test(&test_name!(), test_with_snapshot);
    }

    #[test]
    pub fn show_current_and_new_snapshots_associated_with_test()
    {
        let test_with_changed_snapshot = example_hyp("tests::example_snapshot_changed", SingleTestStatus::Failed);

        show_test(&test_name!(), test_with_changed_snapshot);
    }

    #[test]
    pub fn show_only_new_snapshot_associated_with_test_when_there_is_no_current_snapshot()
    {
        let test_first_run = example_hyp("tests::example_snapshot_only_new", SingleTestStatus::Failed);

        show_test(&test_name!(), test_first_run);
    }

    #[test]
    pub fn show_only_one_snapshot_when_both_current_and_new_are_present_but_identical()
    {
        let test_run_identical_snapshot = example_hyp("tests::example_snapshot_identical", SingleTestStatus::Failed);

        show_test(&test_name!(), test_run_identical_snapshot);
    }

    #[rstest]
    #[case::current_and_new("tests::example_snapshot_changed")]
    #[case::only_new("tests::example_snapshot_only_new")]
    pub fn approving_new_snapshot_emits_event_to_run_test_with_update_snapshots_enabled(#[case] hyp: &str)
    {
        let snapshot_test = example_hyp(hyp, SingleTestStatus::Failed);

        let (test_run_tx, test_run_rx) = Tx::new();

        let mut details_view = DetailsView::new(test_run_tx);
        
        // TODO: Snapshots path to initialize this
        let details = Some(SelectedHyp {
            hyp: snapshot_test,
            snapshot_handles: None
        });

        let ui = |ui: &mut egui::Ui| {
            details_view.ui(ui, &details);
        };
        
        let mut harness = Harness::new_ui(ui);
        
        harness.run();

        let approve = harness.get_by_label("Approve");
        approve.click();
        harness.run();

        let approval_run = test_run_rx.drain().last().unwrap().clone();

        assert_that!(
            &approval_run,
            has_structure!(ChangeEvent::SingleHyp {
                id: eq(HypId::new("example_crate", hyp).unwrap()),
                update_snapshots: eq(true)
            })
        );
    }

    fn show_test(test_name: &str, single_test: SingleTest)
    {
        let mut details_view = DetailsView::new(Tx::stub());

        let details = Some(SelectedHyp {
            hyp: single_test,
            snapshot_handles: None
        });

        let ui = |ui: &mut egui::Ui| {
            details_view.ui(ui, &details);
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

    fn example_hyp(name: &str, status: SingleTestStatus) -> SingleTest
    {
        let id = HypId::new("example_crate", name).unwrap();
        SingleTest::new(id, status, vec![])
    }
}
