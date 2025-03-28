use std::sync::mpsc::channel;
use egui_kittest::Harness;
use crate::views::{DetailsView, View};
use passivate_core::test_run_model::{SingleTest, SingleTestStatus};
use stdext::function_name;

#[test]
pub fn show_a_passing_test() {
    let (sender, receiver)  = channel();
    let mut log_view = DetailsView::new(receiver);

    let ui = |ui: &mut egui::Ui|{
        log_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let passing_test = SingleTest { name: "ExampleTest".to_string(), status: SingleTestStatus::Passed };
    sender.send(passing_test).unwrap();

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
}