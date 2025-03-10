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
                        
                        true
                    },
            TestRunEvent::TestFinished(test) => {
                self.add_or_update_test(test)
            },
            TestRunEvent::NoTests => true
        }
    }

    fn add_or_update_test(&mut self, test: SingleTest) -> bool {
        match self.tests.iter_mut().find(|t| t.name == test.name) {
            Some(existing) => *existing = test,
            None => self.tests.push(test),
        };
        
        true
    }
}
