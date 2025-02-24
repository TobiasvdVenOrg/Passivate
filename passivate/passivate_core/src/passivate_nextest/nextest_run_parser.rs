use std::path::Path;
use crate::test_execution::{SingleTest, SingleTestStatus};

pub fn parse_line(line: &str) -> Option<SingleTest> {
    let trimmed = line.trim();
    
    if trimmed.starts_with("PASS") {
        let name = trimmed.split(" ").last().unwrap_or(trimmed);
        return Some(SingleTest::new(name.to_string(), SingleTestStatus::Passed, Path::new(""), 0));
    } 

    None
}