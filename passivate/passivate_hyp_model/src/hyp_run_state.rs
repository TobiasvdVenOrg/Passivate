use crate::test_run::{BuildFailedTestRun, FailedTestRun};


#[derive(Clone, Debug, PartialEq)]
pub enum HypRunState
{
    FirstRun,
    Idle,
    Building(String),
    Running,
    BuildFailed(BuildFailedTestRun),
    Failed(FailedTestRun)
}