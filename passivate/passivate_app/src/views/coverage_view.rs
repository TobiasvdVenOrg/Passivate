use std::sync::mpsc::Receiver;
use passivate_core::{actors::ActorApi, change_events::ChangeEvent, configuration::PassivateConfig, coverage::{CoverageError, CoverageStatus}};
use crate::views::View;

pub struct CoverageView {
    receiver: Receiver<CoverageStatus>,
    sender: ActorApi<ChangeEvent>,
    status: CoverageStatus
}

impl CoverageView {
    pub fn new(receiver: Receiver<CoverageStatus>, sender: ActorApi<ChangeEvent>) -> CoverageView {
        CoverageView { receiver, sender, status: CoverageStatus::Disabled }
    }

    fn draw_disabled(&mut self, ui: &mut egui_dock::egui::Ui) {
        if ui.button("Enable").clicked() {
            let config = PassivateConfig { coverage_enabled: true };
            self.sender.send(ChangeEvent::Configuration(config));
        }
    }
}

impl View for CoverageView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match self.status {
            CoverageStatus::Disabled => self.draw_disabled(ui),
            CoverageStatus::Error(ref coverage_error) => draw_error(coverage_error)
        };
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}

fn draw_error(error: &CoverageError) {
    match error {
        CoverageError::GrcovNotInstalled(_error_kind) => todo!(),
        CoverageError::FailedToGenerate(_error_kind) => todo!(),
        CoverageError::CleanIncomplete(_error_kind) => todo!(),
        CoverageError::NoProfrawFiles(_error) => todo!(),
        CoverageError::Cancelled(_cancelled) => todo!(),
    };
}