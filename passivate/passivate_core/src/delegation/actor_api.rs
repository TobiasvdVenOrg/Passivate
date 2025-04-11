use std::sync::mpsc::{SendError, Sender};

use super::{actor_event::{ActorEvent, Cancellable}, Cancellation};


pub struct ActorApi<T: Send + 'static> {
    sender: Sender<ActorEvent<T>>
}

impl<T: Send + 'static> ActorApi<T> {
    pub fn new(sender: Sender<ActorEvent<T>>) -> Self {
        Self { sender }
    }

    pub fn send(&self, event: T) -> Result<(), SendError<ActorEvent<T>>> {
        self.sender.send(ActorEvent::Custom(event))
    }

    pub fn send_cancellable(&self, event: T, cancellation: Cancellation) -> Result<(), SendError<ActorEvent<T>>> {
        self.sender.send(ActorEvent::Cancellable(Cancellable { event, cancellation }))
    }
}