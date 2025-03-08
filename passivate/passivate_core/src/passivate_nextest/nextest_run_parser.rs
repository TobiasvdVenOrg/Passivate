use std::path::Path;
use crate::test_execution::{ParseOutput, SingleTest, SingleTestStatus};

pub struct NextestParser;

impl ParseOutput for NextestParser {
    fn parse_line(&self, line: &str) -> Option<SingleTest> {
        let trimmed = line.trim();
        
        if trimmed.starts_with("PASS") {
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            return Some(SingleTest::new(name.to_string(), SingleTestStatus::Passed, Path::new(""), 0));
        } else if trimmed.starts_with("FAIL") {
            let name = trimmed.split(" ").last().unwrap_or(trimmed);
            return Some(SingleTest::new(name.to_string(), SingleTestStatus::Failed, Path::new(""), 0));
        }
    
        None
    }
}
