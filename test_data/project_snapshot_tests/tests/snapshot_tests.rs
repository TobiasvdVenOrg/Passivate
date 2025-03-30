use egui_kittest::Harness;

#[test]
pub fn snapshot_test() {
    let ui = |ui: &mut egui::Ui|{
        ui.heading("This is a heading!");
        ui.label("This is a lable!");
    };

    let mut harness = Harness::new_ui(ui);

    harness.run();
    harness.fit_contents();
    harness.snapshot("example_snapshot");
}
