use egui::Context;
use egui_dock::{DockArea, Style};

use crate::docking::tab_viewer::TabViewer;
use crate::docking::view::View;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    state: egui_dock::DockState<DockWrapper>
}

#[derive(Serialize, Deserialize)]
pub struct DockWrapper
{
    pub id: DockId,

    #[serde(skip)]
    view: Option<Box<dyn View>>
}

impl DockWrapper
{
    fn new(id: DockId, view: Box<dyn View>) -> Self
    {
        Self { id, view: Some(view) }
    }

    pub fn get_view(&mut self) -> &mut Box<dyn View>
    {
        match &mut self.view
        {
            Some(view) => view,
            None => todo!(),
        }
    }
}

impl DockState
{
    pub fn new<TViews>(views: TViews) -> Self
    where
        TViews: Iterator<Item = Box<dyn View>>
    {
        let views = views.map(|view| DockWrapper::new(view.id(), view)).collect();

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

    pub fn views(&self) -> Vec<&DockWrapper> {
        self.state.iter_all_tabs().map(|((_surface, _node), tab)| tab).collect()
    }
}
