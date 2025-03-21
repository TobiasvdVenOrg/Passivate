
#[derive(Clone)]
pub struct PassivateConfig {
    pub coverage_enabled: bool
}

impl Default for PassivateConfig {
    fn default() -> Self {
        Self { coverage_enabled: false }
    }
}
