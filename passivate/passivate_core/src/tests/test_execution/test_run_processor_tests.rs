use std::rc::Rc;

use galvanic_assert::matchers::collection::contains_in_order;
use galvanic_assert::{assert_that, is_variant};
use passivate_delegation::{Cancellation, Rx, Tx};
use rstest::rstest;

use crate::passivate_nextest::NextestParser;
use crate::test_execution::{MockRunTests, TestRunError, TestRunProcessor};
use crate::test_run_model::{SingleTest, SingleTestStatus, TestRun, TestRunState};

struct TestRunIterator
{
    rx: Rx<TestRun>
}

impl Iterator for TestRunIterator
{
    type Item = TestRun;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.rx.next()
    }
}

#[rstest]
pub fn run_completes_when_no_tests_are_found()
{
    let test_run = run("");

    let idle = test_run.last().unwrap().state;

    assert!(matches!(idle, TestRunState::Idle));
}

#[rstest]
pub fn run_transitions_to_idle_after_tests_complete()
{
    let test_run = run("PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6");

    let state = test_run.last().unwrap().state;

    assert_that!(&state, is_variant!(TestRunState::Idle));
}

#[rstest]
pub fn build_output_is_captured_for_building_state()
{
    let mut test_run = run("   Compiling some-dependency v1.2.3");

    let running = test_run.nth(1).unwrap().state;

    assert_that!(&running, is_variant!(TestRunState::Building));
}

#[rstest]
pub fn build_output_is_captured_for_build_failed_state()
{
    let test_run = run("error[E0425]: cannot find value `asdf` in this scope");

    let build_failed = test_run.last().unwrap().state;

    assert_that!(&build_failed, is_variant!(TestRunState::BuildFailed));
}

#[rstest]
pub fn error_output_is_captured_for_failed_test()
{
    let test_output = r#"
    FAIL some_test
    stderr
    a
    b
    "#;
    let test_run = run(test_output);

    let test_run = test_run.last().unwrap();

    assert_that!(
        &test_run.tests,
        contains_in_order(vec![SingleTest::new(
            "some_test".to_string(),
            SingleTestStatus::Failed,
            vec!["a".to_string(), "b".to_string()]
        )])
    );
}

#[rstest]
pub fn error_output_is_recaptured_for_failed_test_on_repeat_runs()
{
    let test_output = r#"
FAIL some_test
stderr
a
b
"#;

    let mut processor = build_processor(test_output);
    let (mut tx, rx) = Tx::new();
    let instrument_coverage = false;

    processor.run_tests(&mut tx, instrument_coverage, Cancellation::default()).unwrap();
    processor.run_tests(&mut tx, instrument_coverage, Cancellation::default()).unwrap();

    let iterator = TestRunIterator { rx };

    let test_run = iterator.last().unwrap();

    assert_that!(
        &test_run.tests,
        contains_in_order(vec![SingleTest::new(
            "some_test".to_string(),
            SingleTestStatus::Failed,
            vec!["a".to_string(), "b".to_string()]
        )])
    );
}

fn run(test_output: &str) -> TestRunIterator
{
    let mut processor = build_processor(test_output);
    let (mut tx, rx) = Tx::new();
    let instrument_coverage = true;

    processor.run_tests(&mut tx, instrument_coverage, Cancellation::default()).unwrap();

    TestRunIterator { rx }
}

fn build_processor(test_output: &str) -> TestRunProcessor
{
    let mut run_tests = MockRunTests::new();
    let test_output = test_output.to_string();
    run_tests.expect_run_tests().returning(move |_instrument_coverage, _cancellation| {
        let iterator = test_output
            .clone()
            .lines()
            .map(|line| Ok(Rc::new(line.to_string())))
            .collect::<Vec<Result<Rc<String>, TestRunError>>>()
            .into_iter();

        Ok(Box::new(iterator))
    });

    let parser = NextestParser::default();

    TestRunProcessor::new(Box::new(run_tests), parser)
}
