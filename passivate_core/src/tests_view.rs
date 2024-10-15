use std::sync::{Arc, RwLock};

pub enum SingleTestStatus {
    Passed,
    Failed
}

pub struct SingleTest {
    pub name: String,
    pub status: SingleTestStatus
}

pub struct TestsStatus {
    pub tests: Vec<SingleTest>,
    pub running: bool
}

impl TestsStatus {
    pub fn new(text: &str) -> Arc<RwLock<TestsStatus>> {
        Arc::new( RwLock::new(TestsStatus { running: false, tests: Vec::new() } ) )
    }
}

pub trait TestsStatusHandler : Send {
    fn refresh(&mut self, tests_status: TestsStatus);
}
