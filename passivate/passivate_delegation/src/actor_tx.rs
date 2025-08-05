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
    implementation: ActorTxImplementation<ActorEvent<T>>,
    name: String
}

impl<T: Send + 'static> ActorTx<T>
{
    pub fn new(sender: Sender<ActorEvent<T>>, name: String) -> Self
    {
        Self {
            implementation: ActorTxImplementation::Channel(sender),
            name
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
            implementation: ActorTxImplementation::Stub,
            name: "stub".to_string()
        }
    }
}

impl<T: Send + 'static> Drop for ActorTx<T>
{
    fn drop(&mut self)
    {
        println!("Drop tx {}", self.name);
    }
}
