use super::{SingleTestStatus, TestCollection, TestRunEvent};

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
    Building(String),
    Running,
    BuildFailed(BuildFailedTestRun),
    Failed(FailedTestRun)
}

#[derive(Clone)]
pub struct TestRun {
    pub state: TestRunState,
    pub tests: TestCollection
}

impl TestRun {
    pub fn from_state(state: TestRunState) -> Self {
        Self { state, tests: TestCollection::default() }
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
                self.tests.add_or_update(test);

                true
            },
            TestRunEvent::NoTests => true,
            TestRunEvent::Compiling(message) => {
                self.state = TestRunState::Building(message.clone());
                true
            },
            TestRunEvent::TestsCompleted => {
                self.state = TestRunState::Idle;
                true
            },
        }
    }
}

impl Default for TestRun {
    fn default() -> Self {
        Self { state: TestRunState::Idle, tests: TestCollection::default() }
    }
}