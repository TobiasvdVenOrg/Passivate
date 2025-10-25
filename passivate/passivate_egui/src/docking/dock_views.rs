use std::collections::HashMap;

use egui_dock::TabViewer;

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

pub struct DockViewer<'a, TView, TState, TCustomUi>
    where 
        TView : View,
        TCustomUi: FnMut(&mut egui::Ui, &mut TView, &mut TState)
{
    pub dock_views: &'a mut DockViews<TView>,
    pub state: &'a mut TState,
    pub custom_ui: TCustomUi
}

impl<TView, TState, TCustomUi> TabViewer for DockViewer<'_, TView, TState, TCustomUi>
    where 
        TView : View,
        TCustomUi: FnMut(&mut egui::Ui, &mut TView, &mut TState)
{
    type Tab = DockId;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText
    {
        let dock_view = self.dock_views.get_view(tab);

        let title = match dock_view
        {
            DockView::View(view) => view.title(),
            DockView::Placeholder(placeholder_view) => placeholder_view.title()
        };

        egui::WidgetText::from(title)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab)
    {
        let dock_view = self.dock_views.get_view(tab);

        match dock_view
        {
            DockView::View(view) => (self.custom_ui)(ui, view, self.state),
            DockView::Placeholder(placeholder_view) => placeholder_view.ui(ui),
        }
    }
}
