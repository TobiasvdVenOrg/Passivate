use super::{SingleTest, TestId};

pub enum TestRunEvent {
    Start,
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