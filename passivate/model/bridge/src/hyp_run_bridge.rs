use passivate_delegation::Tx;

use crate::bridge::Bridge;
use crate::hyp_run_request::{HypRunOptions, HypRunRequest, HypRunRequestKind};

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

impl<TTx, TBridge> RunAllHypsBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypRunRequest<TBridge>>
{
    fn run_all(&self, options: HypRunOptions)
    {
        self.send(HypRunRequest {
            kind: HypRunRequestKind::All,
            options
        });
    }
}

impl<TTx, TBridge> RunSingleHypBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<HypRunRequest<TBridge>>
{
    fn run(&self, hyp_id: TBridge::Id, options: HypRunOptions)
    {
        self.send(HypRunRequest {
            kind: HypRunRequestKind::Single { hyp_id },
            options
        });
    }
}
