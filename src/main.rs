//use passivate_core::add;

mod app;
use notify::*;
use std::path::Path;
use futures::channel::mpsc::channel;
use futures::SinkExt;

fn main() {
    let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");

    let (mut tx, mut rx) = channel(1);
    let mut watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default()).expect("Unable to create watcher.");

    let _ = watcher.watch(Path::new(&path), RecursiveMode::Recursive).expect("Unable to start watching.");

    let eframe_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passivate",
        eframe_options,
        Box::new(|cc| {
            let app = app::App::new(rx, cc.egui_ctx.clone());
            Ok(Box::new(app))
        }),
    ).expect("Unable to open window.");
}
