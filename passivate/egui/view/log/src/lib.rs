use egui::{ScrollArea, Ui};
use passivate_egui_core::log_entry::LogEntry;

pub struct LogView;

impl LogView
{
    pub fn ui(&mut self, ui: &mut Ui, logs: &Vec<LogEntry>)
    {
        ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
            for trace in logs
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
    use passivate_egui_core::log_entry::LogEntry;
    use passivate_hyp_names::test_name;
    use passivate_log::log_message::LogMessage;

    use crate::LogView;

    #[test]
    pub fn show_a_single_log()
    {
        let mut log_view = LogView;
        let logs = vec![LogEntry::from(LogMessage::new_with_timestamp(
            "Hey, this is a log message!".to_string(),
            DateTime::from_timestamp_nanos(1_662_921_288_000_000_000)
        ))];

        let ui = |ui: &mut egui::Ui| {
            log_view.ui(ui, &logs);
        };

        let mut harness = Harness::new_ui(ui);
        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn many_logs_are_scrollable()
    {
        let mut log_view = LogView;
        let mut logs = Vec::new();

        for n in 0 .. 20
        {
            let mut timestamp = DateTime::from_timestamp_nanos(1_662_921_288_000_000_000);
            timestamp += Duration::from_secs(n);

            let example_log = LogMessage::new_with_timestamp("Hey, this is a log message!".to_string(), timestamp);
            logs.push(LogEntry::from(example_log));
        }

        let ui = |ui: &mut egui::Ui| {
            ui.set_max_height(100.0);
            log_view.ui(ui, &logs);
        };

        let mut harness = Harness::new_ui(ui);
        harness.run();

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
