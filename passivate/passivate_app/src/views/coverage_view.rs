use std::sync::mpsc::Receiver;
use passivate_core::coverage::CoverageStatus;
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

        ui.label("COVERAGE");
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}