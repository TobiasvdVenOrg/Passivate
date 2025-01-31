use std::fs;
use std::path::PathBuf;
use eframe::Frame;
use egui::Context;
use passivate_core::error::{ErrorType, NotifyErrorInfo, PassivateError};

pub struct ErrorApp {
    message: String
}

impl eframe::App for ErrorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(self.message.as_str());
        });
    }
}

impl ErrorApp {
    pub fn new(error: PassivateError) -> ErrorApp {
        let message = Self::passivate_error_to_message(error);
        ErrorApp { message }
    }

    pub fn boxed(error: PassivateError) -> Box<ErrorApp> {
        Box::new(Self::new(error))
    }

    fn passivate_error_to_message(passivate_error: PassivateError) -> String {
        match passivate_error.error_type {
            ErrorType::Notify(notify_error) => {
                Self::notify_error_to_message(notify_error)
            }
            ErrorType::MissingArgument(error) => {
                format!("Missing argument: {}", error.argument)
            }
        }
    }

    fn notify_error_to_message(notify_error: NotifyErrorInfo) -> String {
        let absolute_path = Self::try_absolute_path(notify_error.input_path.as_str());

        format!("{}\nInput was: {}\nFull path was: {}\nWorking directory: {}",
                notify_error.notify_error.to_string(),
                notify_error.input_path,
                absolute_path,
                Self::try_working_dir())
    }

    fn try_absolute_path(relative_path: &str) -> String {
        let canonicalize = fs::canonicalize(relative_path);

        match canonicalize {
            Ok(absolute_path) => {
                absolute_path.display().to_string()
            }
            Err(_) => {
                "[Not Found]".to_string()
            }
        }
    }

    fn try_working_dir() -> String {
        let current_dir = std::env::current_dir();

        match current_dir {
            Ok(ok) => {
                ok.display().to_string()
            }
            Err(_) => {
                "[Unknown]".to_string()
            }
        }
    }
}