use std::rc::Rc;
use galvanic_assert::is_variant;
use galvanic_assert::assert_that;
use galvanic_assert::matchers::collection::contains_in_order;
use rstest::rstest;

use crate::delegation::channel;
use crate::delegation::Rx;
use crate::test_run_model::{SingleTest, SingleTestStatus};
use crate::{delegation::Cancellation, configuration::TestRunnerImplementation, test_run_model::{TestRun, TestRunState}};
use crate::test_execution::{build_test_output_parser, MockRunTests, TestRunError, TestRunProcessor};


struct TestRunIterator {
    receiver: Rx<TestRun>
}

impl Iterator for TestRunIterator {
    type Item = TestRun;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.try_iter().next()
    }
}

#[rstest]
#[case::cargo(TestRunnerImplementation::Cargo)]
#[case::nextest(TestRunnerImplementation::Nextest)]
pub fn run_completes_when_no_tests_are_found(#[case] implementation: TestRunnerImplementation) {
    let test_run = run(&implementation, "");    

    let idle = test_run.last().unwrap().state;

    assert!(matches!(idle, TestRunState::Idle));
}

#[rstest]
#[case::cargo(TestRunnerImplementation::Cargo, "test add_2_and_2_is_4 ... ok")]
#[case::nextest(TestRunnerImplementation::Nextest, "PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6")]
pub fn run_transitions_to_idle_after_tests_complete(#[case] implementation: TestRunnerImplementation, #[case] test_output: &str) {
    let test_run = run(&implementation, test_output);

    let state = test_run.last().unwrap().state;

    assert_that!(&state, is_variant!(TestRunState::Idle));
}

#[rstest]
#[case::cargo(TestRunnerImplementation::Cargo, "   Compiling some-dependency v1.2.3")]
#[case::nextest(TestRunnerImplementation::Nextest, "   Compiling some-dependency v1.2.3")]
pub fn build_output_is_captured_for_building_state(#[case] implementation: TestRunnerImplementation, #[case] test_output: &str) {
    let mut test_run = run(&implementation, test_output);

    let running = test_run.nth(1).unwrap().state;

    assert_that!(&running, is_variant!(TestRunState::Building));
}

#[rstest]
#[case::nextest(TestRunnerImplementation::Nextest, "error[E0425]: cannot find value `asdf` in this scope")]
pub fn build_output_is_captured_for_build_failed_state(#[case] implementation: TestRunnerImplementation, #[case] test_output: &str) {
    let test_run = run(&implementation, test_output);

    let build_failed = test_run.last().unwrap().state;

    assert_that!(&build_failed, is_variant!(TestRunState::BuildFailed));
}

#[rstest]
pub fn error_output_is_captured_for_failed_test() {
    let test_output = r#"
    FAIL some_test
    STDERR
    a
    b
    "#;
    let test_run = run(&TestRunnerImplementation::Nextest, test_output);

    let test_run = test_run.last().unwrap();

    assert_that!(&test_run.tests, contains_in_order(vec![
        SingleTest::new(
            "some_test".to_string(),
            SingleTestStatus::Failed,
            vec![
                "a".to_string(),
                "b".to_string()
            ])
    ]));
}

#[rstest]
pub fn error_output_is_recaptured_for_failed_test_on_repeat_runs() {
    let test_output = r#"
FAIL some_test
STDERR
a
b
"#;

    let mut processor = build_processor(&TestRunnerImplementation::Nextest, test_output);    
    let (sender, receiver) = channel();
    let instrument_coverage = false;

    processor.run_tests(&sender, instrument_coverage, Cancellation::default()).unwrap();
    processor.run_tests(&sender, instrument_coverage, Cancellation::default()).unwrap();

    let iterator = TestRunIterator { receiver };

    let test_run = iterator.last().unwrap();

    assert_that!(&test_run.tests, contains_in_order(vec![
        SingleTest::new(
            "some_test".to_string(),
            SingleTestStatus::Failed,
            vec![
                "a".to_string(),
                "b".to_string()
            ])
    ]));
}

fn run(implementation: &TestRunnerImplementation, test_output: &str) -> TestRunIterator {
    let mut processor = build_processor(implementation, test_output);    
    let (sender, receiver) = channel();
    let instrument_coverage = true;

    processor.run_tests(&sender, instrument_coverage, Cancellation::default()).unwrap();

    TestRunIterator { receiver }
}

fn build_processor(implementation: &TestRunnerImplementation, test_output: &str) -> TestRunProcessor {
    let mut run_tests = MockRunTests::new();
    let test_output = test_output.to_string();
    run_tests.expect_run_tests().returning(move |_implementation, _instrument_coverage, _cancellation| {
        let iterator = test_output.clone()
            .lines()
            .map(|line| Ok(Rc::new(line.to_string())))
            .collect::<Vec<Result<Rc<String>, TestRunError>>>()
            .into_iter();

        Ok(Box::new(iterator))
    });

    let parser = build_test_output_parser(implementation);

    TestRunProcessor::new(Box::new(run_tests), parser)
}
