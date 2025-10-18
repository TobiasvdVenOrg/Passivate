use egui::Context;
use egui_dock::{DockArea, Style};

use crate::view::View;

pub struct DockState
{
    state: egui_dock::DockState<DockWrapper>
}

pub struct DockWrapper
{
    view: Box<dyn View>
}

impl DockWrapper
{
    fn new(view: Box<dyn View>) -> Self
    {
        Self { view }
    }
}

impl DockState
{
    pub fn new<TViews>(views: TViews) -> Self
    where
        TViews: Iterator<Item = Box<dyn View>>
    {
        let views = views.map(DockWrapper::new).collect();

        let state = egui_dock::DockState::new(views);

        Self { state }
    }

    pub fn show(&mut self, egui_context: &Context)
    {
        DockArea::new(&mut self.state)
            .style(Style::from_egui(egui_context.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(egui_context, &mut TabViewer);
    }
}

pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer
{
    type Tab = DockWrapper;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText
    {
        tab.view.title().into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab)
    {
        tab.view.ui(ui);
    }
}
