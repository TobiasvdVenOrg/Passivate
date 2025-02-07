use std::sync::{Arc, RwLock};
use passivate_core::test_execution::{HandleTestsStatus, TestsStatus};

pub struct MockHandleTestsStatus {
    status: Arc<RwLock<TestsStatus>>
}

impl MockHandleTestsStatus {
    pub fn new(status: Arc<RwLock<TestsStatus>>) -> MockHandleTestsStatus {
        MockHandleTestsStatus { status }
    }
}

impl HandleTestsStatus for MockHandleTestsStatus {
    fn refresh(&mut self, status: TestsStatus) {
        let mut w = self.status.write().unwrap();
        *w = status;
    }
}