use crossbeam_channel::{SendError, Sender};

use super::{actor_event::ActorEvent, Cancellation};

enum ActorTxImplementation<T: Send + 'static> {
    Channel(Sender<T>),

    Stub
}

impl<T: Send + 'static> Clone for ActorTxImplementation<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Channel(channel) => Self::Channel(channel.clone()),

            Self::Stub => Self::Stub,
        }
    }
}

pub struct ActorTx<T: Send + 'static> {
    implementation: ActorTxImplementation<ActorEvent<T>>
}

impl<T: Send + 'static> Clone for ActorTx<T> {
    fn clone(&self) -> Self {
        Self { implementation: self.implementation.clone() }
    }
}

impl<T: Send + 'static> ActorTx<T> {
    pub fn new(sender: Sender<ActorEvent<T>>) -> Self {
        Self { implementation: ActorTxImplementation::Channel(sender) }
    }

    pub fn send(&self, event: T, cancellation: Cancellation) -> Result<(), SendError<ActorEvent<T>>> {
        match &self.implementation {
            ActorTxImplementation::Channel(sender) => sender.send(ActorEvent { event, cancellation }),

            ActorTxImplementation::Stub => Ok(()),
        }
    }

    pub fn stub() -> Self { 
        Self { implementation: ActorTxImplementation::Stub }
    }
}
