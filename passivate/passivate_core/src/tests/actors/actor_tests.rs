use std::time::Duration;

use crate::delegation::{Actor, Cancellation, Handler, Tx};

#[derive(Default)]
struct ExampleHandler {
    total: i32
}

impl ExampleHandler {
    pub fn get_total(&self) -> i32 {
        self.total
    }
}

impl Handler<i32> for ExampleHandler {
    fn handle(&mut self, event: i32, _cancellation: Cancellation) {
        self.total += event;
    }
}

#[test]
pub fn actor_handles_messages_and_stops_gracefully() {
    let handler = ExampleHandler::default();
    let (actor_tx, mut actor) = Actor::new(handler);

    let tx = Tx::from_actor(actor_tx);
    
    tx.send(16);
    tx.send(32);

    drop(tx);
    let handler = actor.into_inner();

    assert_eq!(48, handler.get_total());
}


struct LoopingHandler;

impl Handler<i32> for LoopingHandler {
    fn handle(&mut self, _event: i32, cancellation: Cancellation) {
        loop {
            std::thread::sleep(Duration::from_millis(100));

            if cancellation.is_cancelled() { return }
        }
    }
}

#[test]
pub fn actor_handle_can_be_cancelled() {
    let handler = LoopingHandler;
    let mut cancellation = Cancellation::default();
    let (tx, _actor) = Actor::new(handler);

    tx.send(64, cancellation.clone()).unwrap();

    cancellation.cancel();

    drop(tx);
}
