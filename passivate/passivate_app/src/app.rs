use std::sync::mpsc::Receiver;
use eframe::Frame;
use egui::{Color32, Context, RichText};
use crate::passivate_notify::{NotifyChangeEvents, NotifyChangeEventsError};
use passivate_core::test_execution::{SingleTestStatus, TestsStatus};
use egui_dock::{DockArea, DockState, Style, TabViewer};

trait View {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui);
    fn title(&self) -> String;
}

struct AppleView;
struct PearView;

impl View for AppleView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        ui.label(format!("Apple!"));
    }
    
    fn title(&self) -> String {
        "A".to_string()
    }
}

impl View for PearView {
    fn ui(&mut self, ui: &mut egui_dock::egui::Ui) {
        ui.label(format!("Pear!"));
    }
    
    fn title(&self) -> String {
        "B".to_string()
    }
}

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
    receiver: Receiver<TestsStatus>,
    change_events: NotifyChangeEvents,
    status: TestsStatus,
    dock_state: DockState<Box<dyn View>>
}

impl App {
    pub fn new(receiver: Receiver<TestsStatus>, change_events: NotifyChangeEvents) -> Self {
        let status = TestsStatus::waiting();
        let views: Vec<Box<dyn View>> = vec!(Box::new(AppleView {}), Box::new(PearView {}));
        let dock_state = DockState::new(views);
        App { receiver, change_events, status, dock_state }
    }

    pub fn boxed(status: Receiver<TestsStatus>, change_events: NotifyChangeEvents) -> Box<App> {
        Box::new(Self::new(status, change_events))
    }

    fn stop(&mut self) -> Result<(), NotifyChangeEventsError> {
        self.change_events.stop()
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
