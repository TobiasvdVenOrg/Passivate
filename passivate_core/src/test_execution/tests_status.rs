use std::sync::{Arc, RwLock};
use crate::test_execution::SingleTest;

pub struct TestsStatus {
    pub tests: Vec<SingleTest>,
    pub running: bool
}

impl TestsStatus {
    pub fn new(text: &str) -> Arc<RwLock<TestsStatus>> {
        Arc::new( RwLock::new(TestsStatus {
            running: false,
            tests: Vec::new()
        }))
    }
}