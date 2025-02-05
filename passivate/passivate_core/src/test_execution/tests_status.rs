use crate::test_execution::SingleTest;

pub struct CompleteTestsStatus {
    pub tests: Vec<SingleTest>
}

pub struct BuildFailureTestsStatus {
    pub message: String
}

pub enum TestsStatus {
    Waiting,
    Running,
    Completed(CompleteTestsStatus),
    BuildFailure(BuildFailureTestsStatus)
}

impl TestsStatus {
    pub fn waiting() -> TestsStatus {
        TestsStatus::Waiting
    }

    pub fn running() -> TestsStatus {
        TestsStatus::Running
    }

    pub fn completed(tests: Vec<SingleTest>) -> TestsStatus {
        TestsStatus::Completed(CompleteTestsStatus { tests })
    }

    pub fn build_failure(message: &str) -> TestsStatus {
        TestsStatus::BuildFailure(BuildFailureTestsStatus { message: message.to_string() })
    }
}