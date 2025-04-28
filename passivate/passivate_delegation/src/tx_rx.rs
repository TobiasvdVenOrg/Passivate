use crossbeam_channel::{Receiver, RecvError, Sender, TryRecvError};

use super::{ActorTx, Cancellation};

enum TxImplementation<T: Send + 'static> {
    Channel(Sender<T>),
    Actor(ActorTx<T>),

    Stub
}

impl<T: Send + 'static> Clone for TxImplementation<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Channel(channel) => Self::Channel(channel.clone()),
            Self::Actor(actor_tx) => Self::Actor(actor_tx.clone()),

            Self::Stub => Self::Stub,
        }
    }
}

pub struct Tx<T: Send + 'static> {
    implementation: TxImplementation<T>
}

impl<T: Send + 'static> Clone for Tx<T> {
    fn clone(&self) -> Self {
        Self { implementation: self.implementation.clone() }
    }
}

impl<T: Send + 'static> Tx<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self { implementation: TxImplementation::Channel(sender)}
    }

    pub fn from_actor(actor_api: ActorTx<T>) -> Self {
        Self { implementation: TxImplementation::Actor(actor_api)}
    }

    pub fn stub() -> Self {
        Self { implementation: TxImplementation::Stub }
    }

    pub fn send(&self, event: T) {
        match &self.implementation {
            TxImplementation::Channel(sender) => sender.send(event).expect("'Tx' failed, receiver is invalid!"),
            TxImplementation::Actor(actor_api) => actor_api.send(event, Cancellation::default()),

            TxImplementation::Stub => { },
        }
    }
}

pub struct Rx<T: Send + 'static> {
    receiver: Receiver<T>
}

impl<T: Send + 'static> Clone for Rx<T> {
    fn clone(&self) -> Self {
        Self { receiver: self.receiver.clone() }
    }
}

impl<T: Send + 'static> Rx<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        Self { receiver }
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn try_iter(&self) -> crossbeam_channel::TryIter<'_, T> {
        self.receiver.try_iter()
    }

    pub fn stub() -> Self {
        Self { receiver: crossbeam_channel::never() }
    }
}
