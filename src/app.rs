use std::process::Command;
use eframe::{Frame};
use egui::Context;
use futures::channel::mpsc::Receiver;
use notify::*;

pub struct App {
    rx: Receiver<Result<Event>>,
    text: String,
    cc: Context,
    w: ReadDirectoryChangesWatcher
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        if let Ok(Some(event)) = self.rx.try_next() {
            match event {
                Ok(_e) => {
                    let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");
                    let output = Command::new("cargo").arg("test").current_dir(path).output().expect("Failed to run tests.");

                    let s = String::from_utf8_lossy(&output.stdout);

                    self.text = s.to_string();
                },
                Err(err) => {
                    println!("error: {:?}", err);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.text.as_str());
        });
    }
}

impl App {
    pub fn new(receiver: Receiver<Result<Event>>, context: Context, watcher: ReadDirectoryChangesWatcher) -> Self {
        App { rx: receiver, text: "Hello!".to_string(), cc: context, w: watcher }
    }
}