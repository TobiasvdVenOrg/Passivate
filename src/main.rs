//use passivate_core::add;

mod app;
use notify::*;
use std::path::Path;

fn main() {
    let path = "F:\\Projects\\passivate\\target";

    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }).expect("Unable to create watcher.");

    let _ = watcher.watch(Path::new(path), RecursiveMode::Recursive).expect("Unable to start watching.");

    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| Ok(Box::new(app::App { }))),
    ).expect("Unable to open window.");
}
