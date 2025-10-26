use eframe::Frame;
use egui::Context;
use egui_dock::{DockArea, Style};
use passivate_egui::docking::dock_views::{DockViewer, DockViews};
use passivate_egui::docking::layout_management::LayoutManagement;
use passivate_egui::passivate_view::PassivateView;

use crate::app_state::AppState;

pub struct App<'a>
{
    layout: LayoutManagement,
    dock_views: DockViews<PassivateView>,
    state: &'a mut AppState<'a>
}

impl<'a> App<'a>
{
    pub fn new(
        layout: LayoutManagement,
        dock_views: DockViews<PassivateView>,
        state: &'a mut AppState<'a>
    ) -> Self
    {
        Self {
            layout,
            dock_views,
            state
        }
    }
}

impl eframe::App for App<'_>
{
    fn update<'a>(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        let layout = self.layout.get_current();

        let mut dock_viewer = DockViewer {
            dock_views: &mut self.dock_views,
            state: self.state,
            custom_ui: AppState::update
        };

        DockArea::new(layout.get_state())
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, &mut dock_viewer);

        ctx.request_repaint();
    }
}
