use crossbeam_channel::Sender;

use super::Cancellation;
use super::actor_event::ActorEvent;

enum ActorTxImplementation<T: Send + 'static>
{
    Channel(Sender<T>),

    Stub
}

pub struct ActorTx<T: Send + 'static>
{
    implementation: ActorTxImplementation<ActorEvent<T>>
}

impl<T: Send + 'static> ActorTx<T>
{
    pub fn new(sender: Sender<ActorEvent<T>>) -> Self
    {
        Self {
            implementation: ActorTxImplementation::Channel(sender)
        }
    }

    pub fn send(&self, event: T, cancellation: Cancellation)
    {
        match &self.implementation
        {
            ActorTxImplementation::Channel(sender) => sender.send(ActorEvent { event, cancellation }).expect("failed to send event to actor"),

            ActorTxImplementation::Stub =>
            {}
        }
    }

    pub fn stub() -> Self
    {
        Self {
            implementation: ActorTxImplementation::Stub
        }
    }
}
