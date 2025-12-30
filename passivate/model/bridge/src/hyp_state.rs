#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum HypState
{
    Passed,
    Failed,
    Unknown,
    Running
}
