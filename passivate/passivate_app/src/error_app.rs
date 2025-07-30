use eframe::Frame;
use egui::Context;

use crate::startup_errors::StartupError;

pub struct ErrorApp
{
    message: String
}

impl eframe::App for ErrorApp
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.message.as_str());
        });
    }
}

impl ErrorApp
{
    pub fn new(error: StartupError) -> ErrorApp
    {
        let message = error.to_string();
        ErrorApp { message }
    }

    pub fn boxed(error: StartupError) -> Box<ErrorApp>
    {
        Box::new(Self::new(error))
    }
}
