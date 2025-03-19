use std::sync::mpsc::channel;
use egui_kittest::Harness;
use crate::views::{TraceView, View};
use passivate_core::cross_cutting::TraceEvent;
use stdext::function_name;

#[test]
pub fn show_a_single_trace() {
    let (sender, receiver)  = channel();
    let mut trace_view = TraceView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        trace_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let example_trace = TraceEvent::new("Hey, this is a trace!");
    sender.send(example_trace).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}