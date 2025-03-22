use std::sync::mpsc::Receiver;
use passivate_core::{actors::ActorApi, configuration::ConfigurationChangeEvent, coverage::{CoverageError, CoverageStatus}};
use crate::views::View;

pub struct CoverageView {
    receiver: Receiver<CoverageStatus>,
    sender: ActorApi<ConfigurationChangeEvent>,
    status: CoverageStatus
}

impl CoverageView {
    pub fn new(receiver: Receiver<CoverageStatus>, sender: ActorApi<ConfigurationChangeEvent>) -> CoverageView {
        CoverageView { receiver, sender, status: CoverageStatus::Disabled }
    }

    fn draw_disabled(&mut self, ui: &mut egui_dock::egui::Ui) {
        if ui.button("Enable").clicked() {
            self.sender.send(ConfigurationChangeEvent::Coverage(true));
        }
    }

    fn draw_error(&mut self, error: CoverageError) {
        match error {
            CoverageError::GrcovNotInstalled(_error_kind) => todo!(),
            CoverageError::FailedToGenerate(_error_kind) => todo!(),
            CoverageError::CleanIncomplete(_error_kind) => todo!(),
            CoverageError::NoProfrawFiles(_error) => todo!(),
            CoverageError::Cancelled(_cancelled) => todo!(),
        };
    }
}

impl View for CoverageView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match self.status {
            CoverageStatus::Disabled => {
                self.draw_disabled(ui);
            },
            CoverageStatus::Error(ref coverage_error) => {
                self.draw_error(coverage_error.clone());
            },
            CoverageStatus::Preparing => {
                ui.heading("Preparing...");
            },
            CoverageStatus::Running => {
                ui.heading("Running...");
            },
            CoverageStatus::Done => {
                ui.heading("Done");
            }
        };
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}
