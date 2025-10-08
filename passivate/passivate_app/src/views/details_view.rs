use camino::Utf8PathBuf;
use egui::{Color32, RichText, TextureHandle, TextureOptions};
use passivate_core::change_events::ChangeEvent;
use passivate_core::configuration::ConfigurationManager;
use passivate_core::test_run_model::{SingleTest, SnapshotError, Snapshots};
use passivate_delegation::{Rx, Tx};
use passivate_hyp_names::hyp_id::HypId;

use super::View;

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

    fn check_for_snapshots(&mut self, ui: &mut egui_dock::egui::Ui, new_test: &Option<SingleTest>)
    {
        if let Some(snapshots) = self.get_snapshots()
            && let Some(new_test) = new_test
        {
            let snapshot = snapshots.from_test(new_test);
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
                hyp_id: new_test.id().clone()
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
                passivate_core::test_run_model::SingleTestStatus::Passed => Color32::GREEN,
                passivate_core::test_run_model::SingleTestStatus::Failed => Color32::RED,
                passivate_core::test_run_model::SingleTestStatus::Unknown => Color32::GRAY
            };

            ui.horizontal(|ui| {
                let text = RichText::new(&single_test.name).size(16.0).color(color);
                ui.heading(text);

                if ui.button("Pin").clicked()
                {
                    self.change_events.send(ChangeEvent::PinHyp { id: single_test.id() });
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
