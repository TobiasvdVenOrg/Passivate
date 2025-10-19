use crate::docking::docking_layout::DockId;

pub trait View
{
    fn id(&self) -> DockId;
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui);
    fn title(&self) -> String;
}
