use std::sync::mpsc::Receiver;

use egui::{Color32, RichText, TextureHandle, TextureOptions};
use passivate_core::test_run_model::{SingleTest, SnapshotError, Snapshots};

use super::View;

struct SnapshotHandles {
    pub current: Option<Result<TextureHandle, SnapshotError>>,
    pub new: Option<Result<TextureHandle, SnapshotError>>
}

pub struct DetailsView {
    receiver: Receiver<Option<SingleTest>>,
    single_test: Option<SingleTest>,
    snapshots: Option<Snapshots>,
    snapshot_handles: Option<SnapshotHandles>
}

impl DetailsView {
    pub fn new(receiver: Receiver<Option<SingleTest>>) -> Self {
        Self { receiver, single_test: None, snapshots: None, snapshot_handles: None }
    }

    pub fn set_snapshots(&mut self, snapshots: Snapshots) {
        self.snapshots = Some(snapshots);
    }
    
    fn check_for_snapshots(&mut self, ui: &mut egui_dock::egui::Ui, new_test: &Option<SingleTest>) {
        if let Some(snapshots) = &self.snapshots {
            if let Some(new_test) = new_test {
                let snapshot = snapshots.from_test(new_test);

                let current = snapshot.current.map(|current| current.map(|s| ui.ctx().load_texture("current_snapshot", s, TextureOptions::LINEAR)));
                let new = snapshot.new.map(|new| new.map(|s| ui.ctx().load_texture("new_snapshot", s, TextureOptions::LINEAR)));

                self.snapshot_handles = Some(SnapshotHandles { current, new });
            }
        }
    }

    fn draw_snapshots(ui: &mut egui_dock::egui::Ui, snapshot_handles: &SnapshotHandles) {
        if let Some(current) = &snapshot_handles.current {
            if snapshot_handles.new.is_some() {
                ui.heading("Current");
            }
            
            Self::draw_snapshot(ui, current);
        }

        if let Some(new) = &snapshot_handles.new {
            ui.heading("New");
            Self::draw_snapshot(ui, new);
        }
    }

    fn draw_snapshot(ui: &mut egui_dock::egui::Ui, snapshot: &Result<TextureHandle, SnapshotError>) {
        match &snapshot {
            Ok(snapshot) => { ui.image(snapshot); },
            Err(error) => {
                let text = RichText::new(error.to_string()).size(16.0).color(Color32::RED);
                ui.heading(text);
            }
        };
    }
}

impl View for DetailsView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_test) = self.receiver.try_recv() {
            self.check_for_snapshots(ui, &new_test);
            self.single_test = new_test;
        }

        if let Some(single_test) = &self.single_test {
            let color = match single_test.status {
                passivate_core::test_run_model::SingleTestStatus::Passed => Color32::GREEN,
                passivate_core::test_run_model::SingleTestStatus::Failed => Color32::RED,
                passivate_core::test_run_model::SingleTestStatus::Unknown => Color32::GRAY,
            };

            let text = RichText::new(&single_test.name).size(16.0).color(color);
            ui.heading(text);
        }

        if let Some(snapshot_handles) = &self.snapshot_handles {
            Self::draw_snapshots(ui, snapshot_handles);
        }
    }

    fn title(&self) -> String {
        "Details".to_string()
    }
}
