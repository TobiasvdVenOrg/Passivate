use std::sync::mpsc::Receiver;
use eframe::Frame;
use egui::Context;
use crate::{passivate_notify::NotifyChangeEvents, views::TestsStatusView};
use crate::views::View;
use passivate_core::test_execution::TestsStatus;
use egui_dock::{DockArea, DockState, Style, TabViewer};

struct MyTabViewer;

impl TabViewer for MyTabViewer {
    type Tab = Box<dyn View>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        tab.title().clone().into()
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }
}

pub struct App {
    _change_events: NotifyChangeEvents,
    dock_state: DockState<Box<dyn View>>
}

impl App {
    pub fn new(receiver: Receiver<TestsStatus>, change_events: NotifyChangeEvents) -> Self {
        let status = TestsStatus::waiting();
        let views: Vec<Box<dyn View>> = vec!(Box::new(TestsStatusView::new(receiver, status)));
        let dock_state = DockState::new(views);
        App { _change_events: change_events, dock_state }
    }

    pub fn boxed(status: Receiver<TestsStatus>, change_events: NotifyChangeEvents) -> Box<App> {
        Box::new(Self::new(status, change_events))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show_leaf_close_all_buttons(false)
            .show(ctx, &mut MyTabViewer);

        ctx.request_repaint();
    }
}
