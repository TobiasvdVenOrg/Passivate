use std::sync::mpsc::Receiver;
use passivate_core::coverage::{CoverageError, CoverageStatus};
use crate::views::View;

pub struct CoverageView {
    receiver: Receiver<CoverageStatus>,
    status: CoverageStatus
}

impl CoverageView {
    pub fn new(receiver: Receiver<CoverageStatus>) -> CoverageView {
        CoverageView { receiver, status: CoverageStatus::Disabled }
    }
}

impl View for CoverageView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match self.status {
            CoverageStatus::Disabled => "Coverage disabled",
            CoverageStatus::Error(ref coverage_error) => {
                match coverage_error {
                    CoverageError::GrcovNotInstalled(_error_kind) => todo!(),
                    CoverageError::FailedToGenerate(_error_kind) => todo!(),
                    CoverageError::CleanIncomplete(_error_kind) => todo!(),
                };
            }
        };
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}