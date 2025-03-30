use std::sync::mpsc::Receiver;

use egui::{Color32, RichText, TextureHandle, TextureOptions};
use passivate_core::test_run_model::{SingleTest, SnapshotError, Snapshots};

use super::View;


pub struct DetailsView {
    receiver: Receiver<Option<SingleTest>>,
    single_test: Option<SingleTest>,
    snapshots: Option<Snapshots>,
    snapshot: Option<Result<TextureHandle, SnapshotError>>
}

impl DetailsView {
    pub fn new(receiver: Receiver<Option<SingleTest>>) -> Self {
        Self { receiver, single_test: None, snapshots: None, snapshot: None }
    }

    pub fn set_snapshots(&mut self, snapshots: Snapshots) {
        self.snapshots = Some(snapshots);
    }
    
    fn draw_snapshot(ui: &mut egui_dock::egui::Ui, snapshot: &Result<TextureHandle, SnapshotError>) {
        match snapshot {
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
            if let Some(snapshots) = &self.snapshots {
                if let Some(new_test) = &new_test {
                    if let Some(snapshot) = snapshots.from_test(new_test) {
                        self.snapshot = Some(snapshot.map(|s| ui.ctx().load_texture("snapshot", s, TextureOptions::LINEAR)));
                    } else {
                        self.snapshot = None;
                    }
                }
            }

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

        if let Some(snapshot) = &self.snapshot {
            Self::draw_snapshot(ui, snapshot);
        }
    }

    fn title(&self) -> String {
        "Details".to_string()
    }
}
