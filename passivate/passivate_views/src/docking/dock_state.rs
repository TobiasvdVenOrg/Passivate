use egui::Context;
use egui_dock::{DockArea, Style};

use crate::docking::tab_viewer::TabViewer;

use serde::{Serialize, Deserialize};

#[derive(Eq, Hash, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DockId(String);

impl From<&str> for DockId
{
    fn from(val: &str) -> Self 
    {
        DockId(val.to_owned())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DockState
{
    state: egui_dock::DockState<DockId>
}

impl DockState
{
    pub fn new(view_ids: impl Iterator<Item = DockId>) -> Self
    {
        let dock_ids = view_ids.collect();

        let state = egui_dock::DockState::new(dock_ids);

        Self { state }
    }

    pub fn show(&mut self, egui_context: &Context, tab_viewer: &mut TabViewer)
    {
        DockArea::new(&mut self.state)
            .style(Style::from_egui(egui_context.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(egui_context, tab_viewer);
    }

    pub fn views(&self) -> Vec<&DockId> {
        self.state.iter_all_tabs().map(|((_surface, _node), tab)| tab).collect()
    }
}
