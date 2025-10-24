use eframe::Frame;
use egui::Context;
use egui_dock::{DockArea, Style, TabViewer};
use passivate_delegation::Rx;
use passivate_hyp_model::hyp_run_events::HypRunEvent;
use passivate_hyp_model::passivate_state::PassivateState;
use passivate_views::docking::dock_views::{DockView, DockViews};
use passivate_views::docking::docking_layout::DockId;
use passivate_views::docking::layout_management::LayoutManagement;
use passivate_views::docking::view::View;
use passivate_views::passivate_view::PassivateView;

pub struct App<'a>
{
    layout: LayoutManagement,
    dock_views: DockViews<PassivateView>,
    state: &'a mut PassivateState,
    hyp_run_rx: Rx<HypRunEvent>
}

impl<'a> App<'a>
{
    pub fn new(
        layout: LayoutManagement,
        dock_views: DockViews<PassivateView>,
        state: &'a mut PassivateState,
        hyp_run_rx: Rx<HypRunEvent>
    ) -> Self
    {
        Self {
            layout,
            dock_views,
            state,
            hyp_run_rx
        }
    }
}

impl eframe::App for App<'_>
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        if let Ok(hyp_run_event) = self.hyp_run_rx.try_recv()
        {
            self.state.hyp_run.update(hyp_run_event);
        }

        let mut layout = self.layout.get_current();

        DockArea::new(&mut layout.get_state())
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, self);

        ctx.request_repaint();
    }
}

impl TabViewer for App<'_>
{
    type Tab = DockId;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText
    {
        let dock_view = self.dock_views.get_view(tab);

        let title = match dock_view
        {
            DockView::View(view) => view.title(),
            DockView::Placeholder(placeholder_view) => placeholder_view.title()
        };

        egui::WidgetText::from(title)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab)
    {
        let dock_view = self.dock_views.get_view(tab);

        match dock_view
        {
            DockView::View(view) => 
            {
                match view
                {
                    PassivateView::Configuration(configuration_view) => todo!(),
                    PassivateView::Coverage(coverage_view) => todo!(),
                    PassivateView::Details(details_view) => todo!(),
                    PassivateView::Log(log_view) => todo!(),
                    PassivateView::TestRun(test_run_view) => todo!(),
                }
            },
            DockView::Placeholder(placeholder_view) => placeholder_view.ui(ui)
        }
    }
}
