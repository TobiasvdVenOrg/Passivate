use super::Cancellation;


pub struct Cancellable<T: Send + 'static> {
    pub event: T,
    pub cancellation: Cancellation
}

pub enum ActorEvent<T: Send + 'static> {
    Custom(T),
    Cancellable(Cancellable<T>),
    Exit
}