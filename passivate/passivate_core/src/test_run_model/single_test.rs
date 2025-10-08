use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};

use super::SingleTestStatus;

#[derive(Clone, Debug, PartialEq)]
pub struct SingleTest
{
    id: HypId,
    pub name: String,
    pub status: SingleTestStatus,
    pub output: Vec<String>
}

impl SingleTest
{
    pub fn new(id: HypId, status: SingleTestStatus, output: Vec<String>) -> Self
    {
        let name = id.get_name(&HypNameStrategy::Default).to_string();

        Self {
            id,
            name,
            status,
            output
        }
    }

    pub fn id(&self) -> HypId
    {
        self.id.clone()
    }
}
