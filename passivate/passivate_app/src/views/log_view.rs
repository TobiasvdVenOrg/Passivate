use std::sync::mpsc::Receiver;
use passivate_core::cross_cutting::LogEvent;
use crate::views::View;

pub struct LogView {
    receiver: Receiver<LogEvent>,
    logs: Vec<LogEntry>
}

struct LogEntry {
    timestamp: String,
    message: String
}

impl LogView {
    pub fn new(receiver: Receiver<LogEvent>) -> Self {
        Self { receiver, logs: vec![] }
    }
}

impl View for LogView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(log) = self.receiver.try_recv() {
            let timestamp_formatted = format!("{}", log.timestamp.format("%H:%M:%S"));
            let entry = LogEntry { timestamp: timestamp_formatted, message: log.message };
            self.logs.push(entry);
        }

        for trace in &self.logs {
            ui.horizontal(|ui| {
                ui.label(&trace.timestamp);
                ui.label(&trace.message);
            });
        }
    }

    fn title(&self) -> String {
        "Log".to_string()
    }
}
