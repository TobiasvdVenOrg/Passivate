use super::DispatchError;

pub trait Dispatch<T> {
    fn dispatch(&self, payload: T) -> Result<(), DispatchError>;
}