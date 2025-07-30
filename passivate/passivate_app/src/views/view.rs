pub trait View
{
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui);
    fn title(&self) -> String;
}
