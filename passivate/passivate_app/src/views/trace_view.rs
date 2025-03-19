use std::sync::mpsc::Receiver;
use passivate_core::cross_cutting::TraceEvent;
use crate::views::View;

pub struct TraceView {
    receiver: Receiver<TraceEvent>,
    traces: Vec<TraceEvent>
}

impl TraceView {
    pub fn new(receiver: Receiver<TraceEvent>) -> Self {
        Self { receiver, traces: vec![] }
    }
}

impl View for TraceView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(trace) = self.receiver.try_recv() {
            self.traces.push(trace);
        }

        for trace in &self.traces {
            ui.label(&trace.message);
        }
    }

    fn title(&self) -> String {
        "Trace".to_string()
    }
}
