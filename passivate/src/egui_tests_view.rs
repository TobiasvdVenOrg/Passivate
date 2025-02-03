use std::sync::{Arc, RwLock};
use egui::Context;
use passivate_core::test_execution::{HandleTestsStatus, TestsStatus};

pub struct EguiTestsView {
    context: Context,
    status: Arc<RwLock<TestsStatus>>
}

impl EguiTestsView {
    pub fn new(ctx: Context, tests_status: Arc<RwLock<TestsStatus>>) -> Self {
        EguiTestsView { context: ctx, status: tests_status }
    }
}

impl HandleTestsStatus for EguiTestsView {
    fn refresh(&mut self, status: TestsStatus) {
        let mut w = self.status.write().unwrap();
        *w = status;

        self.context.request_repaint();
    }
}