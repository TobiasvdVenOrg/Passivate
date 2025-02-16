use super::RunTestsError;

pub trait RunTests {
    fn run_tests(&self) -> Result<String, RunTestsError>;
}