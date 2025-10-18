use crate::view::View;

pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer
{
    type Tab = Box<dyn View>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText
    {
        tab.title().clone().into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab)
    {
        tab.ui(ui);
    }
}
