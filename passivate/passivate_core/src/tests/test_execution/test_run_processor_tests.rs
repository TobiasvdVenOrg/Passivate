use std::{io::Error as IoError, sync::mpsc::{channel, Receiver}};

use rstest::rstest;

use crate::{actors::Cancellation, configuration::TestRunnerImplementation, cross_cutting::stub_log, test_execution::{build_test_output_parser, MockRunTests, TestRunProcessor}, test_run_model::{TestRun, TestRunState}};


struct TestRunIterator {
    receiver: Receiver<TestRun>
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
    let test_run = run(implementation, "");    

    let idle = test_run.last().unwrap().state;

    assert!(matches!(idle, TestRunState::Idle));
}

#[rstest]
#[case::cargo(TestRunnerImplementation::Cargo, "test add_2_and_2_is_4 ... ok")]
#[case::nextest(TestRunnerImplementation::Nextest, "PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6")]
pub fn first_run_transitions_to_running(#[case] implementation: TestRunnerImplementation, #[case] test_output: &str) {
    let test_run = run(implementation, test_output);

    let running = test_run.last().unwrap().state;

    assert!(matches!(running, TestRunState::Running));
}

fn run(implementation: TestRunnerImplementation, test_output: &str) -> TestRunIterator {
    let mut processor = build_processor(implementation, test_output);    
    let (sender, receiver) = channel();

    processor.run_tests(&sender, Cancellation::default()).unwrap();

    TestRunIterator { receiver }
}

fn build_processor(implementation: TestRunnerImplementation, test_output: &str) -> TestRunProcessor {
    let mut run_tests = MockRunTests::new();
    let test_output = test_output.to_string();
    run_tests.expect_run_tests().return_once(move |_implementation| {
        let iterator = test_output
            .lines()
            .map(|line| Ok(line.to_string()))
            .collect::<Vec<Result<String, IoError>>>()
            .into_iter();

        Ok(Box::new(iterator))
    });

    let parser = build_test_output_parser(&implementation);

    TestRunProcessor::new(Box::new(run_tests), parser, stub_log())
}
