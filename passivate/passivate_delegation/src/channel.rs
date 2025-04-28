use super::{Tx, Rx};


pub fn channel<T: Send + 'static>() -> (Tx<T>, Rx<T>) {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let tx = Tx::new(sender);
    let rx = Rx::new(receiver);

    ( tx, rx )
}
