use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};

use crate::single_hyp_status::SingleHypStatus;

#[derive(Clone, Debug, PartialEq)]
pub struct SingleHyp
{
    pub id: HypId,
    pub name: String,
    pub status: SingleHypStatus,
    pub output: Vec<String>
}

impl SingleHyp
{
    pub fn new(id: HypId, status: SingleHypStatus, output: Vec<String>) -> Self
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
