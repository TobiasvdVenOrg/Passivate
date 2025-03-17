use std::sync::mpsc::Sender;

use super::actor_event::ActorEvent;


#[derive(Clone)]
pub struct ActorApi<T: Send + Clone + 'static> {
    sender: Sender<ActorEvent<T>>
}

impl<T: Send + Clone + 'static> ActorApi<T> {
    pub fn new(sender: Sender<ActorEvent<T>>) -> Self {
        Self { sender }
    }

    pub fn send(&self, event: T) {
        self.sender.send(ActorEvent::Custom(event)).unwrap();
    }

    pub fn exit(&self) {
        self.sender.send(ActorEvent::Exit).unwrap();
    }
}