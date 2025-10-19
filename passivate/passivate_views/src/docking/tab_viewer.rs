use std::collections::HashMap;

use crate::docking::docking_layout::DockId;
use crate::docking::view::View;

pub struct TabViewer
{
    views: HashMap<DockId, Box<dyn View>>
}

impl TabViewer
{
    pub fn new<TViews>(views: TViews) -> Self
    where
        TViews: Iterator<Item = Box<dyn View>>
    {
        let views = views.into_iter().map(|view| (view.id(), view)).collect();

        Self { views }
    }

    fn get_view(&mut self, id: &DockId) -> &mut Box<dyn View>
    {
        self.views.entry(id.clone()).or_insert_with(||
        {
            Box::new(PlaceholderView { missing_id: id.clone() })
        })
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

struct PlaceholderView
{
    missing_id: DockId
}

impl View for PlaceholderView
{
    fn id(&self) -> DockId
    {
        self.missing_id.clone()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        ui.heading(format!("failed to resolve dock id: {}", self.missing_id));
    }

    fn title(&self) -> String
    {
        format!("{:?}", self.missing_id).to_owned()
    }
}
