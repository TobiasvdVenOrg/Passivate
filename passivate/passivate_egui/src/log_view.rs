use egui::ScrollArea;
use passivate_delegation::Rx;
use passivate_log::log_message::LogMessage;

pub struct LogView
{
    receiver: Rx<LogMessage>,
    logs: Vec<LogEntry>
}

struct LogEntry
{
    timestamp: String,
    message: String
}

impl LogView
{
    pub fn new(receiver: Rx<LogMessage>) -> Self
    {
        Self { receiver, logs: vec![] }
    }

    pub fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        if let Ok(log) = self.receiver.try_recv()
        {
            let timestamp_formatted = format!("{}", log.timestamp.format("%H:%M:%S"));
            let entry = LogEntry {
                timestamp: timestamp_formatted,
                message: log.content
            };
            self.logs.push(entry);
        }

        ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
            for trace in &self.logs
            {
                ui.horizontal(|ui| {
                    ui.label(&trace.timestamp);
                    ui.label(&trace.message);
                });
            }
        });
    }
}

#[cfg(test)]
mod tests
{
    use std::time::Duration;

    use chrono::DateTime;
    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use passivate_delegation::Tx;
    use passivate_hyp_names::test_name;
    use passivate_log::log_message::LogMessage;

    use crate::log_view::LogView;

    #[test]
    pub fn show_a_single_log()
    {
        let (tx, rx) = Tx::new();
        let mut log_view = LogView::new(rx);

        let ui = |ui: &mut egui::Ui| {
            log_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        let example_log = LogMessage::new_with_timestamp("Hey, this is a log message!".to_string(), DateTime::from_timestamp_nanos(1_662_921_288_000_000_000));
        tx.send(example_log);

        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn many_logs_are_scrollable()
    {
        let (tx, rx) = Tx::new();
        let mut log_view = LogView::new(rx);

        let ui = |ui: &mut egui::Ui| {
            ui.set_max_height(100.0);
            log_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);
        harness.run();
        for n in 0 .. 20
        {
            let mut timestamp = DateTime::from_timestamp_nanos(1_662_921_288_000_000_000);
            timestamp += Duration::from_secs(n);

            let example_log = LogMessage::new_with_timestamp("Hey, this is a log message!".to_string(), timestamp);
            tx.send(example_log);

            harness.run();
        }

        let thing = harness.get_all_by_role(Role::Label).next().unwrap();
        thing.hover();

        harness.run();

        let scrollbar = harness.get_all_by_role(Role::Unknown).next().unwrap();
        scrollbar.hover();

        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }
}