use std::thread;

use passivate_delegation::{CancellableMessage, Cancellation, Rx, Tx};
use passivate_model_bridge::hyp_run_bridge::HypRunTrigger;
use passivate_model_rust::RustBridge;

pub fn change_event_thread<'scope, 'env>(
    scope: &'scope thread::Scope<'scope, 'env>,
    hyp_run_triggers_in: impl Rx<HypRunTrigger<RustBridge>> + 'static,
    hyp_run_triggers_out: impl Tx<CancellableMessage<HypRunTrigger<RustBridge>>> + 'static
)
{
    scope.spawn(move || {
        let mut cancellation = Cancellation::default();

        while let Ok(event) = hyp_run_triggers_in.recv()
        {
            cancellation.cancel();
            cancellation = Cancellation::default();

            hyp_run_triggers_out.send(CancellableMessage {
                message: event,
                cancellation: cancellation.clone()
            });
        }
    });
}
