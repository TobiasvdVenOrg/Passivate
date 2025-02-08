use std::path::Path;
use std::sync::mpsc::{channel};
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::test_execution::{TestRunner, TestsStatus};

#[test]
pub fn change_event_causes_test_run_and_results() {
    let (sender, receiver) = channel();
    let path = Path::new("../sample_project");
    let mut test_runner = TestRunner::new(path, sender);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);

    let running = receiver.recv().unwrap();
    let completed = receiver.recv().unwrap();

    match running {
        TestsStatus::Running => {

        }
        _ => panic!("Expected tests status!")
    }
    match completed {
        TestsStatus::Completed(completed) => {
            assert_eq!(3, completed.tests.len())
        }
        _ => panic!("Expected tests status!")
    }
}
