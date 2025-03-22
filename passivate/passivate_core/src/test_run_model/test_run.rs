use super::{SingleTest, SingleTestStatus, TestRunEvent};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BuildFailedTestRun {
    pub message: String
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct FailedTestRun {
    pub inner_error_display: String
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TestRunState {
    FirstRun,
    Idle,
    Building,
    Running,
    BuildFailed(BuildFailedTestRun),
    Failed(FailedTestRun)
}

#[derive(Clone)]
pub struct TestRun {
    pub state: TestRunState,
    pub tests: Vec<SingleTest>
}

impl TestRun {
    pub fn from_state(state: TestRunState) -> Self {
        Self { state, tests: vec![] }
    }

    pub fn from_failed(failure: FailedTestRun) -> Self {
        Self::from_state(TestRunState::Failed(failure))
    }

    pub fn update(&mut self, event: TestRunEvent) -> bool {
        match event {
            TestRunEvent::Start => {
                        for test in &mut self.tests {
                            test.status = SingleTestStatus::Unknown;
                        }
                        
                        true
                    },
            TestRunEvent::TestFinished(test) => {
                self.state = TestRunState::Running;
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

impl Default for TestRun {
    fn default() -> Self {
        Self { state: TestRunState::Idle, tests: vec![] }
    }
}