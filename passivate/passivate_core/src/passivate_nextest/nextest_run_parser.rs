use crate::{test_execution::ParseOutput, test_run_model::{SingleTest, SingleTestStatus}};

pub struct NextestParser;

impl ParseOutput for NextestParser {
    fn parse_line(&self, line: &str) -> Option<SingleTest> {
        let trimmed = line.trim();
        
        if trimmed.starts_with("PASS") {
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            return Some(SingleTest { name: name.to_string(), status: SingleTestStatus::Passed });
        } else if trimmed.starts_with("FAIL") {
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            return Some(SingleTest { name: name.to_string(), status: SingleTestStatus::Failed });
        }
    
        None
    }
}
