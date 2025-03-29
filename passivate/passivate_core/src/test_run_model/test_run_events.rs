use super::SingleTest;

pub enum TestRunEvent {
    Start,
    Compiling(String),
    TestFinished(SingleTest),
    NoTests
}