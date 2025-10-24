use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};
use passivate_snapshots::SnapshotHandles;

use crate::single_test_status::SingleTestStatus;

#[derive(Clone, Debug, PartialEq)]
pub struct SingleTest
{
    pub id: HypId,
    pub name: String,
    pub status: SingleTestStatus,
    pub output: Vec<String>
}

pub trait SelectHyp
{
    fn select(&mut self, hyp: SingleTest);
}

pub struct SelectedHyp
{
    pub hyp: SingleTest,
    pub snapshot_handles: Option<SnapshotHandles>
}

impl SelectHyp for &mut Option<SelectedHyp>
{
    fn select(&mut self, hyp: crate::single_test::SingleTest)
    {
        let selected = Some(SelectedHyp { hyp, snapshot_handles: None });

        **self = selected;
    }
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
}
