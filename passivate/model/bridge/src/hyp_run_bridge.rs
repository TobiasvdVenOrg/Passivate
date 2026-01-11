use passivate_delegation::{CancellableMessage, Cancellation, Tx};

use crate::bridge::Bridge;
use crate::hyp_run_request::{HypRunOptions, HypRunRequest, HypRunRequestKind};

/// Interface from a session state to start test runs.
#[mockall::automock]
pub trait RunHypsBridge<TBridge: Bridge>
{
    fn run_all(&self, options: HypRunOptions, cancellation: Cancellation);
    fn run_single(&self, hyp: TBridge::Id, options: HypRunOptions, cancellation: Cancellation);
}

impl<TTx, TBridge> RunHypsBridge<TBridge> for TTx
where
    TBridge: Bridge,
    TTx: Tx<CancellableMessage<HypRunRequest<TBridge>>>
{
    fn run_all(&self, options: HypRunOptions, cancellation: Cancellation)
    {
        self.send(CancellableMessage {
            message: HypRunRequest {
                kind: HypRunRequestKind::All,
                options
            },
            cancellation
        });
    }

    fn run_single(&self, hyp_id: TBridge::Id, options: HypRunOptions, cancellation: Cancellation)
    {
        self.send(CancellableMessage {
            message: HypRunRequest {
                kind: HypRunRequestKind::Single { hyp_id },
                options
            },
            cancellation
        });
    }
}
