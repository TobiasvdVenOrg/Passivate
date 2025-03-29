use std::{fs::File, sync::mpsc::Receiver};

use egui::{Color32, ColorImage, RichText, TextureHandle, TextureOptions};
use passivate_core::test_run_model::SingleTest;

use super::View;


pub struct DetailsView {
    receiver: Receiver<SingleTest>,
    single_test: Option<SingleTest>,
    snapshot: Option<TextureHandle>
}

impl DetailsView {
    pub fn new(receiver: Receiver<SingleTest>) -> Self {
        Self { receiver, single_test: None, snapshot: None }
    }
}

impl View for DetailsView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        if let Ok(new_test) = self.receiver.try_recv() {
            if let Some(snaphot) = &new_test.snapshot {
                let decoder = png::Decoder::new(File::open(snaphot).unwrap());
                let mut reader = decoder.read_info().unwrap();
                let mut buffer = vec![0; reader.output_buffer_size()];
                let info = reader.next_frame(&mut buffer).unwrap();

                let dimensions = [info.width as _, info.height as _];
                let buffer_size = info.buffer_size();
                let color_image = match reader.output_color_type().0 {
                    png::ColorType::Grayscale => ColorImage::from_gray(dimensions, &buffer[..buffer_size]),
                    png::ColorType::Rgb => ColorImage::from_rgb(dimensions, &buffer[..buffer_size]),
                    png::ColorType::Indexed => todo!(),
                    png::ColorType::GrayscaleAlpha => todo!(),
                    png::ColorType::Rgba => ColorImage::from_rgba_unmultiplied(dimensions, &buffer[..buffer_size])
                };

                self.snapshot = Some(ui.ctx().load_texture("snapshot", color_image, TextureOptions::LINEAR));
            }

            self.single_test = Some(new_test);
        }

        if let Some(single_test) = &self.single_test {
            let color = match single_test.status {
                passivate_core::test_run_model::SingleTestStatus::Passed => Color32::GREEN,
                passivate_core::test_run_model::SingleTestStatus::Failed => Color32::RED,
                passivate_core::test_run_model::SingleTestStatus::Unknown => Color32::GRAY,
            };

            let text = RichText::new(&single_test.name).size(16.0).color(color);
            ui.heading(text);

            
        }

        if let Some(snapshot) = &self.snapshot {
            ui.image(snapshot);
        }
    }

    fn title(&self) -> String {
        "Details".to_string()
    }
}
