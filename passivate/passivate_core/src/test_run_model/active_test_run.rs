use super::{SingleTest, TestRunEvent};

#[derive(Clone)]
#[derive(Debug)]
pub struct ActiveTestRun {
    pub tests: Vec<SingleTest>
}

impl ActiveTestRun {
    pub fn update(&mut self, event: TestRunEvent) -> bool {
        let changed = match event {
            TestRunEvent::Start => {
                        self.tests.clear();
                        true
                    },
            TestRunEvent::TestFinished(single_test) => {
                self.tests.push(single_test);
                true
            },
            TestRunEvent::NoTests => true
        };

        changed
    }
}