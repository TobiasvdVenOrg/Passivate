use std::{io::Error as IoError, sync::mpsc::channel};

use crate::{configuration::TestRunnerImplementation, test_execution::{build_test_output_parser, MockRunTests, TestRunProcessor}, test_run_model::TestRunState};


#[test]
pub fn run_completes_when_no_tests_are_found() {
    let mut run_tests = MockRunTests::new();
    run_tests.expect_run_tests().return_once(|_implementation| {
        let lines = r#"
        "#;

        let iterator = lines
            .lines()
            .map(|line| Ok(line.to_string()))
            .collect::<Vec<Result<String, IoError>>>()
            .into_iter();

        Ok(Box::new(iterator))
    });

    let parser = build_test_output_parser(&TestRunnerImplementation::Cargo);
    let mut processor = TestRunProcessor::new(Box::new(run_tests), parser);

    let (sender, receiver) = channel();

    processor.run_tests(&sender).unwrap();

    let last = receiver.try_iter().last().unwrap().state;

    assert!(matches!(last, TestRunState::Waiting));
}
