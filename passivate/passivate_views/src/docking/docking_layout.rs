use std::fmt::Display;

use egui::Context;
use egui_dock::{DockArea, DockState, Style};
use serde::{Deserialize, Serialize};

use crate::docking::tab_viewer::TabViewer;

#[derive(Eq, Hash, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DockId(String);

impl From<&str> for DockId
{
    fn from(val: &str) -> Self
    {
        DockId(val.to_owned())
    }
}

impl Display for DockId
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "'{}'", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingLayout
{
    state: DockState<DockId>
}

impl DockingLayout
{
    pub fn new(view_ids: impl Iterator<Item = DockId>) -> Self
    {
        let dock_ids = view_ids.collect();

        let state = DockState::new(dock_ids);

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

    pub fn views(&self) -> Vec<&DockId>
    {
        self.state.iter_all_tabs().map(|((_surface, _node), tab)| tab).collect()
    }
}
