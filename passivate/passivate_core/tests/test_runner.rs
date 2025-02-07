mod test_execution;

use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, RwLock};
use passivate_core::change_events::{ChangeEvent, HandleChangeEvent};
use passivate_core::test_execution::{HandleTestsStatus, TestRunner, TestsStatus};
use test_execution::MockHandleTestsStatus;

#[test]
pub fn change_event_causes_test_run_and_results() {
    let status = Arc::new(RwLock::new(TestsStatus::waiting()));
    let mock_view = Box::new(MockHandleTestsStatus::new(status.clone()));
    let path = Path::new("../sample_project");
    let mut test_runner = TestRunner::new(path, mock_view);

    let mock_event = ChangeEvent { };
    test_runner.handle_event(mock_event);

    let r = status.read().ok().unwrap();
    let result = r.deref();

    match result {
        TestsStatus::Completed(completed) => {
            assert_eq!(3, completed.tests.len())
        }
        _ => panic!("Expected tests status!")
    }
}
