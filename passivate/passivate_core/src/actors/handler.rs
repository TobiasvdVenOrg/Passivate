use super::Cancellation;


pub trait Handler<T: Send + 'static> : Send + 'static {
    fn handle(&mut self, event: T, cancellation: Cancellation);
}