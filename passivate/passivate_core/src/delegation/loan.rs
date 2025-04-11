use super::{ActorApi, Cancellation};

#[mockall::automock]
pub trait Loan<T: Send + 'static> : Send {
    fn send(&self, event: T, cancellation: Cancellation);
}

impl<T: Send + 'static> Loan<T> for ActorApi<T> {
    fn send(&self, event: T, cancellation: Cancellation) {
        self.send_cancellable(event, cancellation).expect("'Loan' failed', receiver is invalid!");
    }
}

pub fn stub_loan<T: Send + 'static>() -> Box<dyn Loan<T>> {
    Box::new(MockLoan::new())
}
