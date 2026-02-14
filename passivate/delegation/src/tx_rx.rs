use thiserror::Error;

#[derive(Error, Debug)]
pub enum RxError
{
    #[error("rx failed")]
    Recv(#[from] crossbeam_channel::RecvError),

    #[error("rx failed")]
    TryRecv(#[from] crossbeam_channel::TryRecvError)
}

#[mockall::automock]
pub trait Tx<T>: Send + Sync
where
    T: Send + Sync + 'static
{
    fn send(&self, message: T);
}

impl<T> Tx<T> for crossbeam_channel::Sender<T>
where
    T: Send + Sync + 'static
{
    fn send(&self, message: T)
    {
        self.send(message).expect("failed to send t")
    }
}

impl<T> Tx<T> for tokio::sync::mpsc::Sender<T>
where
    T: Send + Sync + 'static
{
    fn send(&self, message: T)
    {
        self.blocking_send(message).expect("failed to send t")
    }
}
#[mockall::automock]
pub trait Rx<T>: Send + Sync
where
    T: Send + Sync + 'static
{
    fn recv(&self) -> Result<T, RxError>;
    fn try_recv(&self) -> Result<T, RxError>;
}

impl<T> Rx<T> for crossbeam_channel::Receiver<T>
where
    T: Send + Sync + 'static
{
    fn recv(&self) -> Result<T, RxError>
    {
        Ok(self.recv()?)
    }

    fn try_recv(&self) -> Result<T, RxError>
    {
        Ok(self.try_recv()?)
    }
}
