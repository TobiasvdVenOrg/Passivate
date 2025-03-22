use std::sync::mpsc::Receiver;
use passivate_core::{actors::ActorApi, configuration::ConfigurationEvent, coverage::{CoverageError, CoverageStatus}};
use crate::views::View;

pub struct CoverageView {
    receiver: Receiver<CoverageStatus>,
    sender: ActorApi<ConfigurationEvent>,
    status: CoverageStatus
}

impl CoverageView {
    pub fn new(receiver: Receiver<CoverageStatus>, sender: ActorApi<ConfigurationEvent>) -> CoverageView {
        CoverageView { receiver, sender, status: CoverageStatus::Disabled }
    }

    fn draw_disabled(&mut self, ui: &mut egui_dock::egui::Ui) {
        if ui.button("Enable").clicked() {
            self.sender.send(ConfigurationEvent::Coverage(true));
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
            CoverageStatus::Disabled => self.draw_disabled(ui),
            CoverageStatus::Error(ref coverage_error) => self.draw_error(coverage_error.clone())
        };
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}
