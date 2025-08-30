use mockall::automock;

use super::Cancellation;

type Rx<T> = crossbeam_channel::Receiver<T>;

pub fn tx_rx<T>() -> (impl Tx<T>, Rx<T>)
    where T : Send + Sync
{
    crossbeam_channel::unbounded()
}

pub fn tx_1_rx_2<T>() -> (impl Tx<T>, Rx<T>, Rx<T>)
    where T : Clone + Send + Sync
{
    let (tx1, rx1) = crossbeam_channel::unbounded();
    let (tx2, rx2) = crossbeam_channel::unbounded();
    let tx = Broadcast { txs: vec![tx1, tx2] };

    (tx, rx1, rx2)
}

#[automock]
pub trait Tx<T> : Send + Sync
    where T : Send + Sync
{
    fn send(&self, message: T);
}

pub type BTx<T> = Box<dyn Tx<T>>;

impl<T> Tx<T> for BTx<T>
    where T : Send + Sync
{
    fn send(&self, message:T) {
        (**self).send(message);
    }
}

#[automock]
pub trait TxCancel<T> : Send + Sync
    where T : Send + Sync
{
    fn send(&self, message: T, cancellation: Cancellation);
}

pub struct CancellableMessage<T>
{
    pub message: T,
    pub cancellation: Cancellation
}

pub struct Broadcast<T>
    where T : Send + Sync
{
    txs: Vec<crossbeam_channel::Sender<T>>
}

impl<T> Tx<T> for crossbeam_channel::Sender<T>
    where T : Send + Sync
{
    fn send(&self, message: T)
    {
        self.send(message).expect("failed to send message");
    }
}

impl<T> Tx<T> for Broadcast<T>
where
    T: Clone + Send + Sync
{
    fn send(&self, message: T)
    {
        if let Some((last, txs)) = self.txs.split_last()
        {
            for tx in txs.iter()
            {
                tx.send(message.clone()).expect("failed to send_message");
            }

            last.send(message).expect("failed to send_message");
        }
    }
}

impl<T> TxCancel<T> for crossbeam_channel::Sender<CancellableMessage<T>>
    where T : Send + Sync
{
    fn send(&self, message: T, cancellation: Cancellation)
    {
        self.send(CancellableMessage { message, cancellation }).expect("failed to send_message");
    }
}
