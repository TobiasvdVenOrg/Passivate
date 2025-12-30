use eframe::Frame;
use egui::Context;
use passivate_delegation::Tx;
use passivate_egui_docking::layout_management::LayoutManagement;
use passivate_model_bridge::hyp_run_trigger::HypRunTrigger;
use passivate_model_rust::RustBridge;

use crate::app_state::AppState;

pub struct App<'a>
{
    layout: LayoutManagement,
    state: &'a mut AppState<Tx<HypRunTrigger<RustBridge>>>
}

impl<'a> App<'a>
{
    pub fn new(layout: LayoutManagement, state: &'a mut AppState<Tx<HypRunTrigger<RustBridge>>>) -> Self
    {
        Self { layout, state }
    }

    fn main_update(&mut self, ctx: &Context)
    {
        self.state.update_and_ui(ctx, self.layout.get_current());
    }
}

impl eframe::App for App<'_>
{
    fn update<'a>(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        self.main_update(ctx);
    }
}
