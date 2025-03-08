use crate::test_execution::SingleTest;

#[derive(Clone)]
#[derive(Debug)]
pub struct ActiveTestRun {
    pub tests: Vec<SingleTest>
}

#[derive(Clone)]
#[derive(Debug)]
pub struct BuildFailedTestRun {
    pub message: String
}

#[derive(Clone)]
#[derive(Debug)]
pub struct FailedTestRun {
    pub inner_error_display: String
}

#[derive(Clone)]
#[derive(Debug)]
pub enum TestRun {
    Waiting,
    Starting,
    Active(ActiveTestRun),
    BuildFailed(BuildFailedTestRun),
    Failed(FailedTestRun)
}

impl TestRun {
    
}
