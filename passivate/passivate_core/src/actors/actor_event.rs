
pub enum ActorEvent<T: Send + 'static> {
    Custom(T),
    Exit
}