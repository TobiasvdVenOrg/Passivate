use thiserror::Error;

use super::Cancellation;

#[derive(Error, Debug)]
pub enum RxError
{
    #[error("rx failed")]
    Recv(#[from] crossbeam_channel::RecvError),

    #[error("rx failed")]
    TryRecv(#[from] crossbeam_channel::TryRecvError)
}

#[faux::create]
#[derive(Clone)]
pub struct Tx<T>
{
    tx: crossbeam_channel::Sender<T>
}

#[faux::create]
pub struct Rx<T>
{
    rx: crossbeam_channel::Receiver<T>
}

#[faux::methods]
impl<T> Rx<T>
{
    fn new(rx: crossbeam_channel::Receiver<T>) -> Rx<T>
    {
        Rx { rx }
    }

    pub fn drain(&self) -> Vec<T>
    {
        self.rx.try_iter().collect()
    }

    pub fn recv(&self) -> Result<T, RxError>
    {
        Ok(self.rx.recv()?)
    }

    pub fn try_recv(&self) -> Result<T, RxError>
    {
        Ok(self.rx.try_recv()?)
    }
}

#[faux::methods]
impl<T> IntoIterator for Rx<T>
{
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> <Rx<T> as IntoIterator>::IntoIter
    {
        self.drain().into_iter()
    }
}

#[faux::methods]
impl<T> Tx<T>
{
    pub fn new() -> (Tx<T>, Rx<T>)
    {
        let (tx, rx) = crossbeam_channel::unbounded();

        (Tx { tx }, Rx::new(rx))
    }

    pub fn send(&self, message: T)
    {
        self.tx.send(message).expect("failed to send message");
    }
}

impl<T> Tx<T>
{
    pub fn stub() -> Self
    {
        let mut tx = Tx::faux();
        tx._when_send().then(|_| {});

        tx
    }
}

impl<T: 'static> Rx<T>
{
    pub fn stub() -> Self
    {
        let mut rx = Rx::faux();
        rx._when_recv().then(|_| Err(RxError::Recv(crossbeam_channel::RecvError {})));
        rx._when_try_recv()
            .then(|_| Err(RxError::TryRecv(crossbeam_channel::TryRecvError::Empty)));

        rx
    }
}

pub struct TxCancel<T>(Tx<CancellableMessage<T>>);

impl<T> TxCancel<T>
{
    pub fn send(&self, message: T, cancellation: Cancellation)
    {
        self.0.send(CancellableMessage { message, cancellation });
    }
}

#[derive(Clone)]
pub struct CancellableMessage<T>
{
    pub message: T,
    pub cancellation: Cancellation
}
