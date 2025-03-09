use super::{SingleTest, SingleTestStatus, TestRunEvent};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct ActiveTestRun {
    pub tests: Vec<SingleTest>
}

impl ActiveTestRun {
    pub fn update(&mut self, event: TestRunEvent) -> bool {
        match event {
            TestRunEvent::Start => {
                        for test in &mut self.tests {
                            test.status = SingleTestStatus::Unknown;
                        }
                        
                        !self.tests.is_empty()
                    },
            TestRunEvent::TestFinished(single_test) => {
                self.tests.push(single_test);
                true
            },
            TestRunEvent::NoTests => true
        }
    }
}
