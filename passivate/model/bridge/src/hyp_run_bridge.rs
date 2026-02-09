use passivate_delegation::tx_rx::Tx;

use crate::bridge::Bridge;
use crate::hyp_run_request::{HypRunOptions, HypRunRequest, HypRunRequestKind};

/// Interface from a session state to start test runs.
#[mockall::automock]
pub trait RunHypsBridge<TBridge: Bridge>
{
    fn run_all(&self, options: HypRunOptions);
    fn run_single(&self, hyp: TBridge::Id, options: HypRunOptions);
}

impl<TTx, TBridge> RunHypsBridge<TBridge> for TTx
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

    fn run_single(&self, hyp_id: TBridge::Id, options: HypRunOptions)
    {
        self.send(HypRunRequest {
            kind: HypRunRequestKind::Single { hyp_id },
            options
        });
    }
}
