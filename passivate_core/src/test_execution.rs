use std::process::Command;
use crate::change_events::{ChangeEvent, ChangeEventHandler};
use crate::tests_view::{TestsStatus, TestsView};

pub struct TestExecution {
    view: Box<dyn TestsView>
}

impl TestExecution {
    pub fn new(tests_view: Box<dyn TestsView>) -> Self {
        TestExecution { view: tests_view }
    }
}

impl ChangeEventHandler for TestExecution {
    fn handle_event(&mut self, _event: ChangeEvent) {
        let path = std::env::args().nth(1).expect("Please supply a path to the directory of project's .toml file.");
        let output = Command::new("cargo").arg("test").current_dir(path).output().expect("Failed to run tests.");

        let s = String::from_utf8_lossy(&output.stdout);

        self.view.update(TestsStatus::new(&s));
    }
}
