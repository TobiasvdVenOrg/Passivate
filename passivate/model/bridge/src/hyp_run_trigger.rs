use passivate_delegation::Tx;

use crate::bridge::Bridge;
use crate::hyp_run_bridge::HypRunBridge;

#[derive(Clone, PartialEq, Debug)]
pub enum HypRunTrigger<TBridge: Bridge>
{
    DefaultRun,
    Hyp
    {
        id: TBridge::Id,
        update_snapshots: bool
    }
}

impl<TBridge: Bridge> HypRunBridge for Tx<HypRunTrigger<TBridge>>
{
    fn run_hyps(&self)
    {
        self.send(HypRunTrigger::DefaultRun);
    }
}
