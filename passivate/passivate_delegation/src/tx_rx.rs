use syn::ItemStruct;

use super::Cancellation;

type Rx<T> = crossbeam_channel::Receiver<T>;

enum Mode<T>
{
    Single(crossbeam_channel::Sender<T>),
    Multi(Vec<crossbeam_channel::Sender<T>>)
}

#[faux::create]
pub struct Tx<T>
{
    mode: Mode<T>
}

#[faux::methods]
impl<T> Tx<T>
where
    T: Clone + Send + Sync
{
    pub fn new() -> (Tx<T>, Rx<T>)
    {
        let (tx, rx) = crossbeam_channel::unbounded();

        (Tx { mode: Mode::Single(tx) }, rx)
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

        (tx, rx1, rx2)
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

#[cfg(test)]
impl<T> Tx<T>
where
    T: Clone + Send + Sync
{
    pub fn stub() -> Self
    {
        let mut tx = Tx::faux();
        tx._when_send().then(|_| { });

        tx
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
