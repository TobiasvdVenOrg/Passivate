#[derive(Clone, Debug, PartialEq, Default)]
pub enum HypSessionState
{
    FirstRun,
    #[default]
    Idle,
    Building(String),
    Running,
    BuildFailed(String),
    Failed(String)
}
