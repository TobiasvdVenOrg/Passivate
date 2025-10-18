use eframe::Frame;
use egui::Context;
use egui_dock::{DockArea, DockState, Style, TabViewer};
use passivate_views::{configuration_view::ConfigurationView, coverage_view::CoverageView, details_view::DetailsView, log_view::LogView, test_run_view::TestRunView, view::View};

struct MyTabViewer;

impl TabViewer for MyTabViewer
{
    type Tab = Box<dyn View>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText
    {
        tab.title().clone().into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab)
    {
        tab.ui(ui);
    }
}

pub struct App
{
    dock_state: DockState<Box<dyn View>>
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
        let dock_state = DockState::new(views);
        App { dock_state }
    }
}

impl eframe::App for App
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, &mut MyTabViewer);

        ctx.request_repaint();
    }
}
