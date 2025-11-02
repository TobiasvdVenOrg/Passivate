use crate::hyp_run::HypRun;

#[derive(Debug, Clone, Default)]
pub struct HypSession
{
    last_run: HypRun,
    current_run: HypRun
}

impl HypSession
{
    pub fn last_run(&self) -> &HypRun
    {
        &self.last_run
    }

    pub fn current_run(&self) -> &HypRun
    {
        &self.current_run
    }
}
