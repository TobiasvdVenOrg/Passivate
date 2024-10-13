use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct TestsStatus {
    pub text: String
}

impl TestsStatus {
    pub fn new(text: &str) -> Arc<RwLock<TestsStatus>> {
        Arc::new( RwLock::new(TestsStatus::default()))
    }
}

pub trait TestsStatusHandler : Send {
    fn refresh(&mut self, tests_status: TestsStatus);
}

impl Default for TestsStatus {
    fn default() -> TestsStatus {
        TestsStatus { text: "Hello".to_string() }
    }
}
