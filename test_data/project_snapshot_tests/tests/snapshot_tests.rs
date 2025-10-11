use egui_kittest::{Harness, SnapshotOptions};

#[test]
pub fn snapshot_test() {
    let ui = |ui: &mut egui::Ui|{
        ui.heading("This is a heading!");
        ui.label("This is a label!");
    };

    let mut harness = Harness::new_ui(ui);

    let options = std::env::var("PASSIVATE_SNAPSHOT_DIR")
        .map_or_else(|_| SnapshotOptions::default(), |path| SnapshotOptions::default().output_path(path));

    harness.run();
    harness.fit_contents();
    harness.snapshot_options("example_snapshot", &options);
}

#[test]
pub fn different_snapshot_test() {
    let ui = |ui: &mut egui::Ui|{
        ui.heading("This is a different heading!");
        ui.label("This is a different label!");
    };

    let mut harness = Harness::new_ui(ui);

    let options = std::env::var("PASSIVATE_SNAPSHOT_DIR")
        .map_or_else(|_| SnapshotOptions::default(), |path| SnapshotOptions::default().output_path(path));

    harness.run();
    harness.fit_contents();
    harness.snapshot_options("different_example_snapshot", &options);
}
