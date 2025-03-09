use crate::test_run_model::{ActiveTestRun, SingleTest, SingleTestStatus};


#[test]
pub fn starting_a_run_clears_tests() {
    let mut run = ActiveTestRun { 
        tests: vec![ 
            SingleTest { name: "example_test".to_string(), status: SingleTestStatus::Passed }
            ]
        };

    run.start();

    assert_eq!(0, run.tests.len());
}