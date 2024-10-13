use passivate_core::change_events::{ChangeEvent, ChangeEventHandler};
use passivate_core::test_execution::TestExecution;
use passivate_core::tests_view::{TestsStatus, TestsView};

struct MockTestsView {
    tests_status: TestsStatus
}

impl TestsView for MockTestsView {
    fn update(&mut self, status: TestsStatus) {
        self.tests_status = status;
    }
}

struct MockChangeEvents {

}

impl MockChangeEvents {
    fn go(event: ChangeEvent, mut handler: Box<dyn ChangeEventHandler>) {
        handler.handle_event(event);
    }
}
#[test]
fn change_event_causes_test_run_and_results() {
    let mock_view = MockTestsView { tests_status: TestsStatus::default() };
    let handler = TestExecution::new(Box::new(mock_view));
    //let async_handler = AsyncChangeEventHandler::new(Box::new(handler));
    //let change_events = NotifyChangeEvents::new("F:\\Projects\\passivate\\sample_project", Box::new(async_handler));

    let mock_event = ChangeEvent { };
    MockChangeEvents::go(mock_event, Box::new(handler));

    assert_eq!("", mock_view.tests_status.text);
}
