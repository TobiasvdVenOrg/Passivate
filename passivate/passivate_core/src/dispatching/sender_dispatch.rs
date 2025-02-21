use std::sync::mpsc::Sender;
use super::{Dispatch, DispatchError};

impl<T> Dispatch<T> for Sender<T> {
    fn dispatch(&self, payload: T) -> Result<(), DispatchError> {
        self.send(payload).map_err(|_| DispatchError::ChannelClosed)
    }
}
