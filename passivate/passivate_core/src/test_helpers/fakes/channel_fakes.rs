use std::sync::mpsc::{channel, Receiver, Sender};



pub fn stub_sender<T>() -> Sender<T> {
    let (sender, _receiver) = channel();

    sender
}

pub fn stub_receiver<T>() -> Receiver<T> {
    let (_sender, receiver) = channel();

    receiver
}

pub fn stub_crossbeam_receiver<T>() -> crossbeam_channel::Receiver<T> {
    let (_sender, receiver) = crossbeam_channel::unbounded();

    receiver
}

pub fn stub_crossbeam_sender<T>() -> crossbeam_channel::Sender<T> {
    let (sender, _receiver) = crossbeam_channel::unbounded();

    sender
}
