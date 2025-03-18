use std::time::Duration;

use crate::actors::{Actor, Cancellation, Handler};

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
    let mut actor = Actor::new(handler);

    let api = actor.api();

    api.send(16);
    api.send(32);

    let handler = actor.stop();

    assert_eq!(48, handler.get_total());
}


struct LoopingHandler;

impl Handler<i32> for LoopingHandler {
    fn handle(&mut self, _event: i32, cancellation: Cancellation) {
        loop {
            std::thread::sleep(Duration::from_millis(100));

            if cancellation.is_cancelled() {
                break;
            }
        }
    }
}

#[test]
pub fn actor_handle_can_be_cancelled() {
    let handler = LoopingHandler;
    let mut cancellation = Cancellation::default();
    let mut actor = Actor::new(handler);

    actor.api().send_cancellable(64, cancellation.clone());

    cancellation.cancel();

    actor.stop();
}