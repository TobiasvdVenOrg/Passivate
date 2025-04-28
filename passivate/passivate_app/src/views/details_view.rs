use std::path::PathBuf;

use egui::{Color32, RichText, TextureHandle, TextureOptions};
use passivate_core::configuration::ConfigurationEvent;
use passivate_delegation::{Rx, Tx};
use passivate_core::change_events::ChangeEvent;
use passivate_core::test_run_model::{SingleTest, SnapshotError, Snapshots, TestId};

use super::View;

struct SnapshotHandles {
    pub current: Option<Result<TextureHandle, SnapshotError>>,
    pub new: Option<Result<TextureHandle, SnapshotError>>,
    pub are_identical: bool,
    pub test_id: TestId
}

pub struct DetailsView {
    test_receiver: Rx<Option<SingleTest>>,
    change_events: Tx<ChangeEvent>,
    configuration_receiver: Rx<ConfigurationEvent>,
    single_test: Option<SingleTest>,
    snapshots: Option<Snapshots>,
    snapshot_handles: Option<SnapshotHandles>
}

impl DetailsView {
    pub fn new(
        test_receiver: Rx<Option<SingleTest>>, 
        change_events: Tx<ChangeEvent>, 
        configuration_receiver: Rx<ConfigurationEvent>) -> Self {
        Self { 
            test_receiver,
            change_events,
            configuration_receiver,
            single_test: None, 
            snapshots: None, 
            snapshot_handles: None 
        }
    }

    pub fn get_snapshots(&self) -> Option<Snapshots> {
        self.snapshots.clone()
    }

    pub fn set_snapshots(&mut self, snapshots: Snapshots) {
        self.snapshots = Some(snapshots);
    }
    
    fn check_for_snapshots(&mut self, ui: &mut egui_dock::egui::Ui, new_test: &Option<SingleTest>) {
        if let Some(snapshots) = &self.snapshots {
            if let Some(new_test) = new_test {
                let snapshot = snapshots.from_test(new_test);
                let mut are_identical = false;

                if let (Some(Ok(current)), Some(Ok(new))) = (&snapshot.current, &snapshot.new) {
                    are_identical = current == new;
                }

                let current = snapshot.current.map(|current| current.map(|s| ui.ctx().load_texture("current_snapshot", s, TextureOptions::LINEAR)));
                let new = snapshot.new.map(|new| new.map(|s| ui.ctx().load_texture("new_snapshot", s, TextureOptions::LINEAR)));

                self.snapshot_handles = Some(SnapshotHandles { current, new, are_identical, test_id: new_test.id().clone() });
            }
        }
    }

    fn draw_snapshots(&self, ui: &mut egui_dock::egui::Ui, snapshot_handles: &SnapshotHandles) {
        if let Some(current) = &snapshot_handles.current {
            if snapshot_handles.are_identical {
                Self::draw_snapshot(ui, current);
                return;
            }

            if snapshot_handles.new.is_some() {
                ui.heading("Current");
            }

            Self::draw_snapshot(ui, current);
        }

        if let Some(new) = &snapshot_handles.new {
            ui.horizontal(|ui| {
                ui.heading("New");

                let approve = RichText::new("Approve").size(12.0).color(Color32::GREEN);
                if ui.button(approve).clicked() {
                    self.change_events.send(ChangeEvent::SingleTest { 
                        id: snapshot_handles.test_id.clone(), 
                        update_snapshots: true 
                    });
                }
            });

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
        if let Ok(configuration) = self.configuration_receiver.try_recv() {
            if let Some(snapshots_path) = configuration.new.snapshots_path {
                self.snapshots = Some(Snapshots { snapshot_directory: PathBuf::from(snapshots_path) })
            }
        }

        if let Ok(new_test) = self.test_receiver.try_recv() {
            self.check_for_snapshots(ui, &new_test);
            self.single_test = new_test;
        }

        if let Some(single_test) = &self.single_test {
            let color = match single_test.status {
                passivate_core::test_run_model::SingleTestStatus::Passed => Color32::GREEN,
                passivate_core::test_run_model::SingleTestStatus::Failed => Color32::RED,
                passivate_core::test_run_model::SingleTestStatus::Unknown => Color32::GRAY,
            };

            ui.horizontal(|ui| {
                let text = RichText::new(&single_test.name).size(16.0).color(color);
                ui.heading(text);

                if ui.button("Pin").clicked() {
                    self.change_events.send(ChangeEvent::PinTest { id: single_test.id() });
                }

                if ui.button("Unpin").clicked() {
                    self.change_events.send(ChangeEvent::ClearPinnedTests);
                }
            });

            if !single_test.output.is_empty() {
                ui.add_space(16.0);
    
                for output in &single_test.output {
                    let output_line = RichText::new(output).size(12.0).color(color);
                    ui.label(output_line);
                }
            }
        }

        if let Some(snapshot_handles) = &self.snapshot_handles {
            self.draw_snapshots(ui, snapshot_handles);
        }
    }

    fn title(&self) -> String {
        "Details".to_string()
    }
}
