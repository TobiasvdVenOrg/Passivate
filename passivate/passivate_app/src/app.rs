use eframe::Frame;
use egui::Context;
use passivate_views::docking::layout_management::LayoutManagement;
use passivate_views::docking::tab_viewer::TabViewer;

pub struct App
{
    layout: LayoutManagement,
    tab_viewer: TabViewer
}

impl App
{
    pub fn new(layout: LayoutManagement, tab_viewer: TabViewer) -> Self
    {
        Self { layout, tab_viewer }
    }
}

impl eframe::App for App
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        self.layout.show_current(ctx, &mut self.tab_viewer);

        ctx.request_repaint();
    }
}
