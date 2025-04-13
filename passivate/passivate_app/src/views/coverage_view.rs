use egui::{collapsing_header::CollapsingState, Color32, RichText};
use passivate_core::{configuration::ConfigurationChangeEvent, coverage::CoverageStatus, delegation::{Rx, Tx}, passivate_grcov::CovdirJson};
use crate::views::View;

pub struct CoverageView {
    receiver: Rx<CoverageStatus>,
    configuration: Tx<ConfigurationChangeEvent>,
    status: CoverageStatus
}

impl CoverageView {
    pub fn new(receiver: Rx<CoverageStatus>, configuration: Tx<ConfigurationChangeEvent>) -> CoverageView {
        CoverageView { receiver, configuration, status: CoverageStatus::Disabled }
    }

    fn draw_coverage(ui: &mut egui_dock::egui::Ui, coverage: &CovdirJson, id: egui::Id) {
        if coverage.children.as_ref().is_none_or(|children|children.is_empty()) {
            ui.horizontal(|ui| {
                ui.label(&coverage.name);
                ui.label(format!("{}%", &coverage.coverage_percent));
            });
        } else {
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
    }

    fn draw_disabled(&self, ui: &mut egui_dock::egui::Ui) {
        ui.heading("Code coverage is disabled");

        if ui.button("Enable").clicked() {
            self.configuration.send(ConfigurationChangeEvent::Coverage(true));
        }
    }
}

impl View for CoverageView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(status) = self.receiver.try_recv() {
            self.status = status;
        }

        match &self.status {
            CoverageStatus::Disabled => self.draw_disabled(ui),
            CoverageStatus::Error(ref coverage_error) => { 
                let text = RichText::new(coverage_error).size(16.0).color(Color32::RED);
                ui.heading(text); 
            },
            CoverageStatus::Preparing => { ui.heading("Preparing..."); },
            CoverageStatus::Running => { ui.heading("Running..."); },
            CoverageStatus::Done(json) => Self::draw_coverage(ui, json, egui::Id::new(format!("root{}", json.name)))
        };
    }

    fn title(&self) -> String {
        "Coverage".to_string()
    }
}
