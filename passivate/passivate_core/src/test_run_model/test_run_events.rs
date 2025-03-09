use super::SingleTest;

pub enum TestRunEvent {
    Start,
    TestFinished(SingleTest),
    NoTests
}