use crate::actors::{Actor, Handler};

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
    fn handle(&mut self, event: i32) {
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