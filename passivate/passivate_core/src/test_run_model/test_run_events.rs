use super::{SingleTest, TestId};

#[derive(Debug, Clone)]
pub enum TestRunEvent
{
    Start,
    StartSingle
    {
        test: TestId,
        clear_tests: bool
    },
    Compiling(String),
    BuildError(String),
    TestFinished(SingleTest),
    TestsCompleted,
    NoTests,
    ErrorOutput
    {
        test: TestId,
        message: String
    }
}
