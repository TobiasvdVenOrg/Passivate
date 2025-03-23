use std::sync::mpsc::Receiver;
use egui::collapsing_header::CollapsingState;
use passivate_core::{actors::ActorApi, configuration::ConfigurationChangeEvent, coverage::{CoverageError, CoverageStatus}, passivate_grcov::CovdirJson};
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

    fn draw_coverage(ui: &mut egui_dock::egui::Ui, coverage: &CovdirJson, id: egui::Id) {  
        let default_open = false;
        CollapsingState::load_with_default_open(ui.ctx(), id, default_open)
            .show_header(ui, |ui| {
                ui.label(&coverage.name);
                ui.label(format!("{}%", &coverage.coverage_percent));
            })
            .body(|ui| {
                if let Some(children) = &coverage.children {
                    for child in children.values() {
                        let hierarchical_id = egui::Id::new(format!("{:?}{}", id, child.name));
                        Self::draw_coverage(ui, child, hierarchical_id);
                    }
                } else {
                    ui.label("None");
                }
            });
    }

    fn draw_disabled(&self, ui: &mut egui_dock::egui::Ui) {
        ui.heading("Code coverage is disabled");

        if ui.button("Enable").clicked() {
            self.sender.send(ConfigurationChangeEvent::Coverage(true));
        }
    }

    fn draw_error(&self, error: CoverageError) {
        match error {
            CoverageError::GrcovNotInstalled(_error_kind) => todo!(),
            CoverageError::FailedToGenerate(_error_kind) => todo!(),
            CoverageError::CleanIncomplete(_error_kind) => todo!(),
            CoverageError::NoProfrawFiles(_error) => todo!(),
            CoverageError::Cancelled(_cancelled) => todo!(),
            CoverageError::CovdirRead(_error_kind) => todo!(),
            CoverageError::CovdirParse(_) => todo!(),
        };
    }
}

impl View for CoverageView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match &self.status {
            CoverageStatus::Disabled => self.draw_disabled(ui),
            CoverageStatus::Error(ref coverage_error) => self.draw_error(coverage_error.clone()),
            CoverageStatus::Preparing => { ui.heading("Preparing..."); },
            CoverageStatus::Running => { ui.heading("Running..."); },
            CoverageStatus::Done(json) => Self::draw_coverage(ui, json, egui::Id::new(format!("root{}", json.name)))
        };
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}
