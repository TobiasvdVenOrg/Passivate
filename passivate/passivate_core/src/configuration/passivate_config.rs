#[derive(Clone, Default, PartialEq, Debug)]
pub struct PassivateConfig
{
    pub coverage_enabled: bool,
    pub snapshots_path: Option<String>
}
