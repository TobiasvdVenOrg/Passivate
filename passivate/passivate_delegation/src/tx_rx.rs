use bus::Bus;
use crossbeam_channel::{Receiver, RecvError, Sender, TryRecvError};

use super::{ActorTx, Cancellation};

pub fn tx_1_rx_1<T: Send + 'static>() -> (Tx<T>, Rx<T>) {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let tx = Tx { implementation: TxImplementation::Channel(sender) };
    let rx = Rx { receiver };

    ( tx, rx )
}

pub fn tx_1_rx_2<T: Send + 'static>() -> (Tx<T>, Rx<T>, Rx<T>) {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let tx = Tx { implementation: TxImplementation::Channel(sender) };
    let rx1 = Rx { receiver: receiver.clone() };
    let rx2 = Rx { receiver };

    ( tx, rx1, rx2 )
}

enum TxImplementation<T: Send + 'static> {
    Channel(Sender<T>),
    Actor(ActorTx<T>),
    Bus(Bus<T>),

    Stub
}

pub struct Tx<T: Send + 'static> {
    implementation: TxImplementation<T>
}

impl<T: Send + 'static> Tx<T> {
    pub fn stub() -> Self {
        Self { implementation: TxImplementation::Stub }
    }

    pub fn send(&mut self, event: T) {
        match &mut self.implementation {
            TxImplementation::Channel(sender) => sender.send(event).expect("'Tx' failed, receiver is invalid!"),
            TxImplementation::Actor(actor_api) => actor_api.send(event, Cancellation::default()),
            TxImplementation::Bus(bus) => bus.broadcast(event),
            TxImplementation::Stub => { },
        }
    }
}

pub struct Rx<T: Send + 'static> {
    receiver: Receiver<T>
}

impl<T: Send + 'static> Rx<T> {
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

impl<T: Send + 'static> From<ActorTx<T>> for Tx<T> {
    fn from(actor_tx: ActorTx<T>) -> Self {
        Tx { implementation: TxImplementation::Actor(actor_tx) }
    }
}
