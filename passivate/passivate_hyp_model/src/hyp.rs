use std::fmt::Display;

use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};

use crate::hyp_state::HypState;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hyp
{
    pub id: HypId,
    pub name: String,
    state: HypState,
    pub output: Vec<String>
}

impl Hyp
{
    pub fn new(id: HypId, state: HypState) -> Self
    {
        let name = id.get_name(&HypNameStrategy::Default).to_string();

        Self {
            id,
            name,
            state,
            output: Vec::new()
        }
    }

    pub fn current_state(&self) -> HypState
    {
        self.state
    }

    pub fn add_output(&mut self, output: String)
    {
        self.output.push(output);
    }
}

impl Display for Hyp
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{} - {:?}", self.name, self.current_state())
    }
}
