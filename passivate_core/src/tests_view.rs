use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub enum SingleTestStatus {
    Passed,
    Failed
}

pub struct SingleTest {
    pub name: String,
    pub status: SingleTestStatus,
    pub file: PathBuf,
    pub line: u32
}

impl SingleTest {
    pub fn new(name: String, status: SingleTestStatus, file: &Path, line: u32) -> SingleTest {
        SingleTest {
            name,
            status,
            file: file.to_path_buf(),
            line
        }
    }
}

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

pub trait TestsStatusHandler : Send {
    fn refresh(&mut self, tests_status: TestsStatus);
}
