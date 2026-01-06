use eframe::Frame;
use egui::Context;
use passivate_configuration::configuration_source::FileConfigurationSource;
use passivate_egui_docking::docking_layout::DockingLayout;
use passivate_egui_docking::layout_management::LayoutManagement;

use crate::app_state::AppState;

pub struct App<'a>
{
    layout: LayoutManagement<FileConfigurationSource<DockingLayout>>,
    state: &'a mut AppState
}

impl<'a> App<'a>
{
    pub fn new(layout: LayoutManagement<FileConfigurationSource<DockingLayout>>, state: &'a mut AppState) -> Self
    {
        Self { layout, state }
    }

    fn main_update(&mut self, ctx: &Context)
    {
        self.state.update_app(ctx, self.layout.get_current());
    }
}

impl eframe::App for App<'_>
{
    fn update<'a>(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        self.main_update(ctx);
    }
}
