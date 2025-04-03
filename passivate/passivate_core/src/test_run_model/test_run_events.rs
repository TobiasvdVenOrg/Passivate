use super::{SingleTest, TestId};

pub enum TestRunEvent {
    Start,
    StartSingle {
        test: TestId,
        clear_tests: bool
    },
    Compiling(String),
    BuildError(String),
    TestFinished(SingleTest),
    TestsCompleted,
    NoTests,
    ErrorOutput {
        test: TestId,
        message: String
    }
}