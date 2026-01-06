use passivate_delegation::Tx;

use crate::bridge::Bridge;

/// Interface from a session state to start test runs.
#[mockall::automock]
pub trait RunAllHypsBridge<TBridge: Bridge>
{
    fn run_all(&self, options: HypRunOptions);
}

pub trait RunSingleHypBridge<TBridge: Bridge>
{
    fn run(&self, hyp: TBridge::Id, options: HypRunOptions);
}

pub trait UpdateSnapshotsBridge<TBridge: Bridge>
{
    fn run_and_update_snapshots(&self, hyp: TBridge::Id);
}

pub struct HypRunTrigger<TBridge: Bridge>
{
    pub kind: HypRunTriggerKind<TBridge>,
    pub options: HypRunOptions
}

#[derive(Default)]
pub struct HypRunOptions
{
    pub update_snapshots: bool,
    pub compute_coverage: bool
}

#[derive(Clone, PartialEq, Debug)]
pub enum HypRunTriggerKind<TBridge: Bridge>
{
    All,
    Single
    {
        hyp_id: TBridge::Id
    }
}

impl<TTx, TBridge> RunAllHypsBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypRunTrigger<TBridge>>
{
    fn run_all(&self, options: HypRunOptions)
    {
        self.send(HypRunTrigger {
            kind: HypRunTriggerKind::All,
            options
        });
    }
}

impl<TTx, TBridge> RunSingleHypBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypRunTrigger<TBridge>>
{
    fn run(&self, hyp_id: TBridge::Id, options: HypRunOptions)
    {
        self.send(HypRunTrigger {
            kind: HypRunTriggerKind::Single { hyp_id },
            options
        });
    }
}
