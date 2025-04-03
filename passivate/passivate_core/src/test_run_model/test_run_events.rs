use super::{SingleTest, TestId};

pub enum TestRunEvent {
    Start,
    StartSingle(TestId),
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