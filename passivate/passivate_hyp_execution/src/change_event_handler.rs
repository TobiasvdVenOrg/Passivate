use std::thread::{self, JoinHandle};

use passivate_delegation::{CancellableMessage, Cancellation, Rx, Tx};
use passivate_hyp_model::hyp_run_trigger::HypRunTrigger;

pub fn change_event_thread(rx: Rx<HypRunTrigger>, tx: Tx<CancellableMessage<HypRunTrigger>>) -> JoinHandle<()>
{
    thread::spawn(move || {
        let mut cancellation = Cancellation::default();

        while let Ok(event) = rx.recv()
        {
            cancellation.cancel();
            cancellation = Cancellation::default();

            tx.send(CancellableMessage { message: event, cancellation: cancellation.clone() });
        }
    })
}
