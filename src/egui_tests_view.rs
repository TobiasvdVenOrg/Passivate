use eframe::Frame;
use egui::Context;
use passivate_core::tests_view::{TestsStatus, TestsView};

struct EguiTestsView {
    context: Context,
    status: TestsStatus
}

impl EguiTestsView {
    pub fn new(ctx: Context) -> Self {
        EguiTestsView { context: ctx, status: TestsStatus::default() }
    }
}

impl TestsView for EguiTestsView {
    fn update(&mut self, status: TestsStatus) {
        self.status = status;
        self.context.request_repaint();
    }
}

impl eframe::App for EguiTestsView {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.status.text.as_str());
        });
    }
}
