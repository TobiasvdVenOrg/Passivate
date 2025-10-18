use egui::Context;
use egui_dock::{DockArea, Style};

use crate::{docking::tab_viewer::TabViewer, view::View};

pub struct DockState
{
    state: egui_dock::DockState<Box<dyn View>>
}

impl DockState
{
    pub fn new<TViews>(views: TViews) -> Self
    where
        TViews: Iterator<Item = Box<dyn View>>
    {
        let views: Vec<Box<dyn View>> = views.collect();

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
