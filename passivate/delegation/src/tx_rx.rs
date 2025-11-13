use thiserror::Error;

use super::Cancellation;

#[derive(Clone)]
enum Mode<T>
{
    Single(crossbeam_channel::Sender<T>),
    Multi(Vec<crossbeam_channel::Sender<T>>)
}

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
    mode: Mode<T>
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
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> <Rx<T> as IntoIterator>::IntoIter
    {
        self.drain().into_iter()
    }
}

#[faux::methods]
impl<T> Tx<T>
where
    T: Clone + Send + Sync
{
    pub fn new() -> (Tx<T>, Rx<T>)
    {
        let (tx, rx) = crossbeam_channel::unbounded();

        (Tx { mode: Mode::Single(tx) }, Rx::new(rx))
    }

    pub fn multi_2() -> (Tx<T>, Rx<T>, Rx<T>)
    where
        T: Clone + Send
    {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        let tx = Tx {
            mode: Mode::Multi(vec![tx1, tx2])
        };

        (tx, Rx::new(rx1), Rx::new(rx2))
    }

    pub fn send(&self, message: T)
    {
        match &self.mode
        {
            Mode::Single(sender) => sender.send(message).expect("failed to send single message"),
            Mode::Multi(senders) =>
            {
                if let Some((last, txs)) = senders.split_last()
                {
                    for tx in txs.iter()
                    {
                        tx.send(message.clone()).expect("failed to send multi message");
                    }

                    last.send(message).expect("failed to send multi message");
                }
            }
        }
    }
}

impl<T> Tx<T>
where
    T: Clone + Send + Sync
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
where
    T: Clone + Send + Sync
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
