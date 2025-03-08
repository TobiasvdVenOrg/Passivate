use crate::test_execution::SingleTest;

#[derive(Clone)]
#[derive(Debug)]
pub struct CompleteTestsStatus {
    pub tests: Vec<SingleTest>
}

#[derive(Clone)]
#[derive(Debug)]
pub struct BuildFailureTestsStatus {
    pub message: String
}

#[derive(Clone)]
#[derive(Debug)]
pub struct RunTestsErrorStatus {
    pub inner_error_display: String
}

#[derive(Clone)]
#[derive(Debug)]
pub enum TestRun {
    Waiting,
    Running,
    Completed(CompleteTestsStatus),
    BuildFailure(BuildFailureTestsStatus),
    RunTestsError(RunTestsErrorStatus)
}

impl TestRun {
    pub fn waiting() -> TestRun {
        TestRun::Waiting
    }

    pub fn completed(tests: Vec<SingleTest>) -> TestRun {
        TestRun::Completed(CompleteTestsStatus { tests })
    }

    pub fn build_failure(message: &str) -> TestRun {
        TestRun::BuildFailure(BuildFailureTestsStatus { message: message.to_string() })
    }
}
