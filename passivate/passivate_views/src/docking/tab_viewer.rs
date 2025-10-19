use std::collections::HashMap;

use crate::docking::{dock_state::DockId, view::View};

pub struct TabViewer
{
    views: HashMap<DockId, Box<dyn View>>
}

impl TabViewer
{
    pub fn new<TViews>(views: TViews) -> Self
    where 
        TViews : Iterator<Item = Box<dyn View>>
    {
        let views = views.into_iter()
            .map(|view| (view.id(), view))
            .collect();

        Self { views }
    }

    fn get_view(&mut self, id: &DockId) -> &mut Box<dyn View>
    {
        self.views.get_mut(id).unwrap()
    }
}

impl egui_dock::TabViewer for TabViewer
{
    type Tab = DockId;

    // Implement this as lookup, not the view being a field of the Tab (DockWrapper can just be a DockId)
    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText
    {
        self.get_view(tab).title().into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab)
    {
        self.get_view(tab).ui(ui);
    }
}
