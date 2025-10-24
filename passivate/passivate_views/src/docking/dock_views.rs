use std::collections::HashMap;

use crate::docking::{docking_layout::DockId, view::View};

pub enum DockView<TView: View>
{
    View(TView),
    Placeholder(PlaceholderView)
}

pub struct DockViews<TView: View>
{
    views: HashMap<DockId, DockView<TView>>
}

impl<TView: View> DockViews<TView>
{
    pub fn new<TViews>(views: TViews) -> Self
    where 
        TViews : IntoIterator<Item = TView>
    {
        let views = views.into_iter().map(|view| (view.id(), DockView::View(view))).collect();

        Self { views }
    }

    pub fn get_view(&mut self, id: &DockId) -> &mut DockView<TView>
    {
        self.views.entry(id.clone()).or_insert_with(||
        {
            DockView::Placeholder(PlaceholderView { missing_id: id.clone() })
        })
    }
}

pub struct PlaceholderView
{
    missing_id: DockId
}

impl PlaceholderView
{
    pub fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        ui.heading(format!("failed to resolve dock id: {}", self.missing_id));
    }
}

impl View for PlaceholderView
{
    fn id(&self) -> DockId
    {
        self.missing_id.clone()
    }

    fn title(&self) -> String
    {
        format!("{:?}", self.missing_id).to_owned()
    }
}
