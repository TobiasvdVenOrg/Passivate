use crate::{configuration::TestRunnerImplementation, passivate_cargo::CargoTestParser, passivate_nextest::NextestParser, test_run_model::TestRunEvent};

#[mockall::automock]
pub trait ParseOutput {
    fn parse_line(&self, line: &str) -> Option<TestRunEvent>;
    fn get_implementation(&self) -> TestRunnerImplementation;
}

pub fn build_test_output_parser(implementation: &TestRunnerImplementation) -> Box<dyn ParseOutput + Send> {
    match implementation {
        TestRunnerImplementation::Cargo => Box::new(CargoTestParser),
        TestRunnerImplementation::Nextest => Box::new(NextestParser),
    }
}

pub fn mock_parse_output() -> Box<MockParseOutput> {
    Box::new(MockParseOutput::new())
}

pub fn stub_parse_output() -> Box<MockParseOutput> {
    let mut mock = mock_parse_output();
    mock.expect_parse_line().returning(|_line| { None });
    mock.expect_get_implementation().returning(|| { TestRunnerImplementation::Cargo });

    mock
}
