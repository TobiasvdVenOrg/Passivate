
#[derive(Clone, Debug, PartialEq)]
pub enum HypRunState
{
    FirstRun,
    Idle,
    Building(String),
    Running,
    BuildFailed(String),
    Failed(String)
}