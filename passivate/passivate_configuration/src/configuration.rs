
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Configuration
{
    pub coverage_enabled: bool,
    pub snapshots_path: Option<String>
}
