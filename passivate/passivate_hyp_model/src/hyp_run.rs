use std::collections::HashMap;
use std::collections::hash_map::Values;

use passivate_hyp_names::hyp_id::HypId;

use crate::single_hyp::SingleHyp;

#[derive(Debug, Clone, Default)]
pub struct HypRun
{
    pub hyps: HashMap<HypId, SingleHyp>
}

impl HypRun
{
    pub fn add_hyp(&mut self, hyp: SingleHyp) -> Option<SingleHyp>
    {
        self.hyps.insert(hyp.id.clone(), hyp)
    }

    pub fn hyps(&self) -> Values<'_, HypId, SingleHyp>
    {
        self.hyps.values()
    }

    pub fn find_hyp(&self, hyp_id: &HypId) -> Option<&SingleHyp>
    {
        self.hyps.get(hyp_id)
    }
}
