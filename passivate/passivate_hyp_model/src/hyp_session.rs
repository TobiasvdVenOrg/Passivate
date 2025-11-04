use crate::hyp_run::HypRun;
use crate::hyp_run_events::{HypRunChange, HypRunEvent};

#[derive(Debug, Clone, Default)]
pub struct HypSession
{
    last_run: HypRun,
    current_run: HypRun
}

impl HypSession
{
    pub fn new(last_run: HypRun, current_run: HypRun) -> Self
    {
        Self { last_run, current_run }
    }

    pub fn last_run(&self) -> &HypRun
    {
        &self.last_run
    }

    pub fn current_run(&self) -> &HypRun
    {
        &self.current_run
    }

    pub fn update(&mut self, event: HypRunEvent) -> Option<HypRunChange<'_>>
    {
        self.current_run.update(event)
    }
}
