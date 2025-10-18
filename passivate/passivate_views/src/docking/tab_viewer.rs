use crate::docking::dock_state::DockWrapper;

pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer
{
    type Tab = DockWrapper;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText
    {
        tab.get_view().title().into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab)
    {
        tab.get_view().ui(ui);
    }
}
