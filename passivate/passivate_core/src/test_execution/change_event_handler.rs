use std::thread::{self, JoinHandle};

use passivate_delegation::{CancellableMessage, Cancellation, Rx, Tx};

use crate::change_events::ChangeEvent;

pub fn change_event_thread(rx: Rx<ChangeEvent>, tx: Tx<CancellableMessage<ChangeEvent>>) -> JoinHandle<()>
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
