use std::fmt::Display;

use egui_dock::DockState;
use serde::{Deserialize, Serialize};

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

    pub fn views(&self) -> Vec<&DockId>
    {
        self.state.iter_all_tabs().map(|((_surface, _node), tab)| tab).collect()
    }

    pub fn dock_state(&mut self) -> &mut DockState<DockId>
    {
        &mut self.state
    }
}
