use crate::delegation::{Cancellation, Handler, Loan};
use crate::change_events::ChangeEvent;

pub struct ChangeEventHandler {
    decoratee: Box<dyn Loan<ChangeEvent>>,
    cancellation: Cancellation
}

impl ChangeEventHandler {
    pub fn new(decoratee: Box<dyn Loan<ChangeEvent>>) -> Self {
        Self { decoratee, cancellation: Cancellation::default() }
    }
}

impl Handler<ChangeEvent> for ChangeEventHandler {
    fn handle(&mut self, event: ChangeEvent, _cancellation: Cancellation) {
        self.cancellation.cancel();
        self.cancellation = Cancellation::default();

        self.decoratee.send(event, self.cancellation.clone());
    }
}
