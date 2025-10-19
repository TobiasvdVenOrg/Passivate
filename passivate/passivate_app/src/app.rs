use eframe::Frame;
use egui::Context;
use passivate_views::configuration_view::ConfigurationView;
use passivate_views::coverage_view::CoverageView;
use passivate_views::details_view::DetailsView;
use passivate_views::docking::dock_state::DockState;
use passivate_views::docking::tab_viewer::TabViewer;
use passivate_views::docking::view::View;
use passivate_views::log_view::LogView;
use passivate_views::test_run_view::TestRunView;

pub struct App
{
    dock_state: DockState,
    tab_viewer: TabViewer
}

impl App
{
    pub fn new(test_run_view: TestRunView, details_view: DetailsView, coverage_view: CoverageView, configuration_view: ConfigurationView, log_view: LogView) -> Self
    {
        let views: Vec<Box<dyn View>> = vec![
            Box::new(test_run_view),
            Box::new(details_view),
            Box::new(coverage_view),
            Box::new(configuration_view),
            Box::new(log_view),
        ];

        let dock_state = DockState::new(views.iter().map(|view| view.id()));
        let tab_viewer = TabViewer::new(views.into_iter());

        App { dock_state, tab_viewer }
    }
}

impl eframe::App for App
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        self.dock_state.show(ctx, &mut self.tab_viewer);

        ctx.request_repaint();
    }
}
