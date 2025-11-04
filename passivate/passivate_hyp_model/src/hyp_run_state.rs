#[derive(Clone, Debug, PartialEq, Default)]
pub enum HypRunState
{
    FirstRun,
    #[default]
    Idle,
    Building(String),
    Running,
    BuildFailed(String),
    Failed(String)
}
