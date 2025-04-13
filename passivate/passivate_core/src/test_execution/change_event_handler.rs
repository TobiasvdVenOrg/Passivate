use crate::delegation::{ActorTx, Cancellation, Handler};
use crate::change_events::ChangeEvent;

pub struct ChangeEventHandler {
    actor: ActorTx<ChangeEvent>,
    cancellation: Cancellation
}

impl ChangeEventHandler {
    pub fn new(actor: ActorTx<ChangeEvent>) -> Self {
        Self { actor, cancellation: Cancellation::default() }
    }
}

impl Handler<ChangeEvent> for ChangeEventHandler {
    fn handle(&mut self, event: ChangeEvent, _cancellation: Cancellation) {
        self.cancellation.cancel();
        self.cancellation = Cancellation::default();

        self.actor.send(event, self.cancellation.clone()).expect("failed to send change event to actor!");
    }
}
