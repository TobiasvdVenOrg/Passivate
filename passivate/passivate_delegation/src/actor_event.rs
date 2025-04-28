use super::Cancellation;

pub struct ActorEvent<T: Send + 'static> {
    pub event: T,
    pub cancellation: Cancellation
}
