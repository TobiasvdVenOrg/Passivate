use crate::test_run_model::{ActiveTestRun, SingleTest, SingleTestStatus, TestRunEvent};

#[test]
pub fn starting_a_run_makes_all_test_statuses_unknown() {
    let mut run = ActiveTestRun { 
        tests: vec![ 
            SingleTest { name: "example_test".to_string(), status: SingleTestStatus::Passed }
            ]
        };

    assert!(run.update(TestRunEvent::Start));
    assert!(run.tests.iter().all(|t| t.status == SingleTestStatus::Unknown));
}