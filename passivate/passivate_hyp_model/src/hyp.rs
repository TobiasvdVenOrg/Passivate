use std::fmt::Display;

use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};

use crate::hyp_state::HypState;

#[derive(Clone, Debug, PartialEq)]
pub struct Hyp
{
    pub id: HypId,
    pub name: String,
    pub status: HypState,
    pub output: Vec<String>
}

impl Hyp
{
    pub fn new(id: HypId, status: HypState, output: Vec<String>) -> Self
    {
        let name = id.get_name(&HypNameStrategy::Default).to_string();

        Self {
            id,
            name,
            status,
            output
        }
    }
}

impl Display for Hyp
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{} - {:?}", self.name, self.status)
    }
}
