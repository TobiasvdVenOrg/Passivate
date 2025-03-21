use super::Cancellation;

#[mockall::automock]
pub trait Handler<T: Send + 'static> : Send + 'static {
    fn handle(&mut self, event: T, cancellation: Cancellation);
}

pub fn mock_handler<T: Send + 'static>() -> MockHandler<T> {
    let mut mock_handler = MockHandler::new();
    mock_handler.expect_handle().returning(|_, _| {});

    mock_handler
}