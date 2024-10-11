//use passivate_core::add;

mod app;
use notify::*;
use std::{path::Path, time::Duration};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use std::thread;

fn main() {
    let path = "F:\\Projects\\passivate\\target";
    println!("Hello");

    thread::spawn(move || {
        let mut watcher = notify::recommended_watcher(|res| {
            match res {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }).expect("Unable to create watcher");

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        let w = watcher.watch(Path::new(path), RecursiveMode::Recursive);

        if w.is_err() {
            println!("watch error: {:?}", w.unwrap_err());
        }

        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

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
    ).expect("red");
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
