use std::process::Command;
use std::time::Duration;
use eframe::{Frame};
use egui::Context;
use futures::channel::mpsc::Receiver;
use notify::*;

pub struct App {
    rx: Receiver<Result<Event>>,
    text: String,
    cc: Context
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        if let Ok(event) = self.rx.try_next() {

            println!("event: {:?}", event);
            let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");
            let output = Command::new("cargo").arg("test").current_dir(path).output().expect("Failed to run tests.");

            println!("Tests run!");
            let s = String::from_utf8_lossy(&output.stdout);

            self.text = s.to_string();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.text.as_str());
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl App {
    pub fn new(receiver: Receiver<Result<Event>>, context: Context) -> Self {
        App { rx: receiver, text: "Hello!".to_string(), cc: context }
    }

    pub fn refresh(&self) {
        self.cc.request_repaint()
    }
}