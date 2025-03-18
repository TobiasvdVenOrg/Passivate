use std::sync::mpsc::Sender;

use super::{actor_event::{ActorEvent, Cancellable}, Cancellation};


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

    pub fn send_cancellable(&self, event: T, cancellation: Cancellation) {
        self.sender.send(ActorEvent::Cancellable(Cancellable { event, cancellation })).unwrap();
    }

    pub fn exit(&self) {
        self.sender.send(ActorEvent::Exit).unwrap();
    }
}