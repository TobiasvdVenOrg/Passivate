use camino::Utf8PathBuf;
use egui::{Color32, RichText, TextureHandle, TextureOptions};
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::change_events::ChangeEvent;
use passivate_delegation::{Rx, Tx};
use passivate_hyp_model::single_test::SingleTest;
use passivate_hyp_model::single_test_status::SingleTestStatus;
use passivate_hyp_names::hyp_id::HypId;
use passivate_snapshots::snapshots::{SnapshotError, Snapshots};

use crate::docking::docking_layout::DockId;
use crate::docking::view::View;

struct SnapshotHandles
{
    pub current: Option<Result<TextureHandle, SnapshotError>>,
    pub new: Option<Result<TextureHandle, SnapshotError>>,
    pub are_identical: bool,
    pub hyp_id: HypId
}

pub struct DetailsView
{
    test_receiver: Rx<Option<SingleTest>>,
    change_events: Tx<ChangeEvent>,
    configuration: ConfigurationManager,
    single_test: Option<SingleTest>,
    snapshot_handles: Option<SnapshotHandles>
}

impl DetailsView
{
    pub fn new(test_receiver: Rx<Option<SingleTest>>, change_events: Tx<ChangeEvent>, configuration: ConfigurationManager) -> Self
    {
        Self {
            test_receiver,
            change_events,
            configuration,
            single_test: None,
            snapshot_handles: None
        }
    }

    pub fn get_snapshots(&self) -> Option<Snapshots>
    {
        let snapshots_path = self.configuration.get(|c| c.snapshots_path.clone());

        snapshots_path.map(|path| Snapshots::new(Utf8PathBuf::from(path)))
    }

    fn check_for_snapshots(&mut self, ui: &mut egui_dock::egui::Ui, new_hyp: &Option<SingleTest>)
    {
        if let Some(snapshots) = self.get_snapshots()
            && let Some(new_hyp) = new_hyp
        {
            let snapshot = snapshots.from_hyp(&new_hyp.id);
            let mut are_identical = false;

            if let (Some(Ok(current)), Some(Ok(new))) = (&snapshot.current, &snapshot.new)
            {
                are_identical = current == new;
            }

            let current = snapshot
                .current
                .map(|current| current.map(|s| ui.ctx().load_texture("current_snapshot", s, TextureOptions::LINEAR)));
            let new = snapshot.new.map(|new| new.map(|s| ui.ctx().load_texture("new_snapshot", s, TextureOptions::LINEAR)));

            self.snapshot_handles = Some(SnapshotHandles {
                current,
                new,
                are_identical,
                hyp_id: new_hyp.id.clone()
            });
        }
    }

    fn draw_snapshots(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        if let Some(snapshot_handles) = &mut self.snapshot_handles
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

impl View for DetailsView
{
    fn id(&self) -> DockId
    {
        "details_view".into()
    }
    
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        if let Ok(new_test) = self.test_receiver.try_recv()
        {
            self.check_for_snapshots(ui, &new_test);
            self.single_test = new_test;
        }

        if let Some(single_test) = &self.single_test
        {
            let color = match single_test.status
            {
                SingleTestStatus::Passed => Color32::GREEN,
                SingleTestStatus::Failed => Color32::RED,
                SingleTestStatus::Unknown => Color32::GRAY
            };

            ui.horizontal(|ui| {
                let text = RichText::new(&single_test.name).size(16.0).color(color);
                ui.heading(text);

                if ui.button("Pin").clicked()
                {
                    self.change_events.send(ChangeEvent::PinHyp { id: single_test.id.clone() });
                }

                if ui.button("Unpin").clicked()
                {
                    self.change_events.send(ChangeEvent::ClearPinnedHyps);
                }
            });

            if !single_test.output.is_empty()
            {
                ui.add_space(16.0);

                for output in &single_test.output
                {
                    let output_line = RichText::new(output).size(12.0).color(color);
                    ui.label(output_line);
                }
            }
        }

        self.draw_snapshots(ui);
    }

    fn title(&self) -> String
    {
        "Details".to_string()
    }
}

#[cfg(test)]
mod tests
{
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use galvanic_assert::matchers::*;
    use galvanic_assert::*;
    use passivate_configuration::configuration::PassivateConfiguration;
    use passivate_configuration::configuration_manager::ConfigurationManager;
    use passivate_core::change_events::ChangeEvent;
    use passivate_delegation::{Rx, Tx};
    use passivate_hyp_model::single_test::SingleTest;
    use passivate_hyp_model::single_test_status::SingleTestStatus;
    use passivate_hyp_model::test_run::TestRun;
    use passivate_hyp_model::hyp_run_events::HypRunEvent;
    use passivate_hyp_names::hyp_id::HypId;
    use passivate_hyp_names::test_name;
    use passivate_testing::path_resolution::test_data_path;
    use rstest::*;

    use crate::details_view::DetailsView;
    use crate::docking::view::View;
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
        let (details_tx, details_rx) = Tx::new();

        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());

        let mut details_view = DetailsView::new(details_rx, Tx::stub(), configuration);

        let mut test_run = TestRun::default();
        test_run.tests.add(example_hyp("tests::example_test", SingleTestStatus::Failed));
        let mut test_run_view = TestRunView::new(test_run, Rx::stub(), details_tx);

        let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui| {
            test_run_view.ui(ui);
        });

        let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui| {
            details_view.ui(ui);
        });

        test_run_ui.run();

        let test_entry = test_run_ui.get_by_label("example_test");
        test_entry.click();

        test_run_ui.run();
        details_ui.run();

        details_ui.fit_contents();
        details_ui.snapshot(&test_name!());
    }

    #[test]
    pub fn when_a_test_is_selected_and_then_changes_status_the_details_view_also_updates()
    {
        let (test_run_tx, test_run_rx) = Tx::new();
        let (details_tx, details_rx) = Tx::new();
        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());
        let mut details_view = DetailsView::new(details_rx, Tx::stub(), configuration);
        let mut test_run_view = TestRunView::with_default_status(test_run_rx, details_tx);

        let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui| {
            test_run_view.ui(ui);
        });

        let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui| {
            details_view.ui(ui);
        });

        test_run_tx.send(HypRunEvent::TestFinished(example_hyp("tests::example_test", SingleTestStatus::Failed)));

        test_run_ui.run();

        let test_entry = test_run_ui.get_by_label("example_test");
        test_entry.click();

        test_run_ui.run();
        details_ui.run();

        test_run_tx.send(HypRunEvent::TestFinished(example_hyp("tests::example_test", SingleTestStatus::Passed)));

        test_run_ui.run();
        details_ui.run();

        details_ui.fit_contents();
        details_ui.snapshot(&test_name!());
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

        let (details_tx, details_rx) = Tx::new();
        let (test_run_tx, test_run_rx) = Tx::new();
        let configuration = get_configuration_with_example_snapshots_path();

        let mut details_view = DetailsView::new(details_rx, test_run_tx, configuration);

        let ui = |ui: &mut egui::Ui| {
            details_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        details_tx.send(Some(snapshot_test));
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
        let (tx, rx) = Tx::new();
        let configuration = get_configuration_with_example_snapshots_path();

        let mut details_view = DetailsView::new(rx, Tx::stub(), configuration);

        let ui = |ui: &mut egui::Ui| {
            details_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        tx.send(Some(single_test));

        harness.run();
        harness.fit_contents();
        harness.snapshot(test_name);
    }

    fn get_configuration_with_example_snapshots_path() -> ConfigurationManager
    {
        ConfigurationManager::new(
            PassivateConfiguration {
                snapshots_path: Some(test_data_path().join("example_snapshots").to_string()),
                ..PassivateConfiguration::default()
            },
            Tx::stub()
        )
    }

    fn example_hyp(name: &str, status: SingleTestStatus) -> SingleTest
    {
        let id = HypId::new("example_crate", name).unwrap();
        SingleTest::new(id, status, vec![])
    }
}