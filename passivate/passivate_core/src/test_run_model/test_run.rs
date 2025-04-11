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
                self.state = TestRunState::Running;
                for test in &mut self.tests {
                    test.status = SingleTestStatus::Unknown;
                    test.output.clear();
                }

                true
            },
            TestRunEvent::StartSingle { test, clear_tests } => {
                if let Some(mut test) = self.tests.find(&test) {
                    self.state = TestRunState::Running;
                    test.status = SingleTestStatus::Unknown;
                    test.output.clear();

                    if clear_tests {
                        self.tests.clear();
                        self.tests.add_or_update(test);
                    }

                    return true;
                }
                
                false
            },
            TestRunEvent::TestFinished(mut test) => {
                self.state = TestRunState::Running;

                let existing = self.tests.find(&test.id());
                
                match existing {
                    Some(existing) => {
                        test.output = existing.output;
                        self.tests.add_or_update(test);
                    },
                    None => self.tests.add_or_update(test),
                };

                true
            },
            TestRunEvent::NoTests => {
                self.state = TestRunState::Idle;
                true
            }
            TestRunEvent::Compiling(message) => {
                self.state = TestRunState::Building(message.clone());
                true
            },
            TestRunEvent::TestsCompleted => {
                self.state = TestRunState::Idle;
                true
            },
            TestRunEvent::BuildError(message) => {
                self.state = TestRunState::BuildFailed(BuildFailedTestRun { message });
                true
            },
            TestRunEvent::ErrorOutput { test, message } => {
                if !message.is_empty() {
                    if let Some(mut updated_test) = self.tests.find(&test) {
                        updated_test.output.push(message);
                        self.tests.add_or_update(updated_test);
                        return true;
                    }
                }

                false
            }
        }
    }
}

impl Default for TestRun {
    fn default() -> Self {
        Self { state: TestRunState::Idle, tests: TestCollection::default() }
    }
}