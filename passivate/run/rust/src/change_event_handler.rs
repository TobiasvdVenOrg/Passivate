use std::thread::{self, JoinHandle};

use passivate_delegation::{CancellableMessage, Cancellation, Rx, Tx};
use passivate_model_bridge::hyp_run_trigger::HypRunTrigger;
use passivate_model_rust::RustBridge;

pub fn change_event_thread(
    rx: Rx<HypRunTrigger<RustBridge>>,
    tx: Tx<CancellableMessage<HypRunTrigger<RustBridge>>>
) -> JoinHandle<()>
{
    thread::spawn(move || {
        let mut cancellation = Cancellation::default();

        while let Ok(event) = rx.recv()
        {
            cancellation.cancel();
            cancellation = Cancellation::default();

            tx.send(CancellableMessage {
                message: event,
                cancellation: cancellation.clone()
            });
        }
    })
}
