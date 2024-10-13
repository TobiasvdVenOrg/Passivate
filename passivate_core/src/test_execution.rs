use std::process::Command;
use crate::change_events::{ChangeEvent, ChangeEventHandler};
use crate::tests_view::{TestsStatus, TestsStatusHandler};

pub struct TestExecution {
    tests_status_handler: Box<dyn TestsStatusHandler>
}

impl TestExecution {
    pub fn new(tests_status_handler: Box<dyn TestsStatusHandler>) -> Self {
        TestExecution { tests_status_handler }
    }
}

impl ChangeEventHandler for TestExecution {
    fn handle_event(&mut self, _event: ChangeEvent) {
        self.tests_status_handler.refresh(TestsStatus { text: "RUNNING TESTS".to_string() });

        let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");
        let output = Command::new("cargo").arg("test").current_dir(path).output().expect("Failed to run tests.");

        let s = String::from_utf8_lossy(&output.stdout);

        self.tests_status_handler.refresh(TestsStatus { text: s.to_string() });
    }
}
