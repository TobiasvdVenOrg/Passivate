use crate::{configuration::TestRunnerImplementation, test_execution::ParseOutput, test_run_model::{SingleTest, SingleTestStatus, TestRunEvent}};

pub struct NextestParser;

impl ParseOutput for NextestParser {
    fn parse_line(&self, line: &str) -> Option<TestRunEvent> {
        let trimmed = line.trim();
        
        if trimmed.starts_with("PASS") {
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            let test = SingleTest::new(name.to_string(), SingleTestStatus::Passed);

            return Some(TestRunEvent::TestFinished(test));
        } else if trimmed.starts_with("FAIL") {
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            let test = SingleTest::new(name.to_string(), SingleTestStatus::Failed);
            
            return Some(TestRunEvent::TestFinished(test));
        } else if trimmed.starts_with("Compiling") {
            return Some(TestRunEvent::Compiling(trimmed.to_string()));
        }
    
        None
    }
    
    fn get_implementation(&self) -> TestRunnerImplementation {
        TestRunnerImplementation::Nextest
    }
}
