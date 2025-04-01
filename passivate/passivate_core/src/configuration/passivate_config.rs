

#[derive(Clone)]
pub struct PassivateConfig {
    pub coverage_enabled: bool,
    pub snapshots_path: Option<String>
}

impl Default for PassivateConfig {
    fn default() -> Self {
        Self { 
            coverage_enabled: false,
            snapshots_path: None 
        }
    }
}
