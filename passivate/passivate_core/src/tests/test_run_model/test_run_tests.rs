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

#[test]
pub fn second_run_updates_unknown_tests_when_completed() {
    let example_test = SingleTest { name: "example_test".to_string(), status: SingleTestStatus::Passed };
    let mut run = ActiveTestRun { 
        tests: vec![ 
            example_test.clone()
            ]
        };

    assert!(run.update(TestRunEvent::Start));
    assert!(run.update(TestRunEvent::TestFinished(example_test.clone())));

    assert_eq!(1, run.tests.len());
    assert_eq!(example_test, run.tests[0]);
}