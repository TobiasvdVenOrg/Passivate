use std::time::Duration;

use passivate_delegation::{Actor, ActorTx, Cancellation, Handler};

struct LoopingHandler;

impl Handler<i32> for LoopingHandler {
    fn handle(&mut self, _event: i32, cancellation: Cancellation) {
        loop {
            std::thread::sleep(Duration::from_millis(100));

            if cancellation.is_cancelled() 
            { 
                return 
            }
        }
    }
}

#[test]
pub fn actor_handle_can_be_cancelled() {
    let handler = LoopingHandler;
    let mut cancellation = Cancellation::default();
    let (_actor, tx) = Actor::new(handler);
    
    send(tx, cancellation.clone());
    
    cancellation.cancel();
}

fn send(tx: ActorTx<i32>, cancellation: Cancellation) {
    tx.send(64, cancellation);
}
