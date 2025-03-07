use std::{ffi::OsStr, path::Path};
use crate::test_execution::{ParseOutput, SingleTest, SingleTestStatus};

pub struct CargoTestParser;

impl ParseOutput for CargoTestParser {
    fn parse_line(&self, line: &str) -> Option<SingleTest> {
        let line = line.trim();

        if line.starts_with("test") {
            if let Some((test, result)) = split_and_trim(line) {
                let status = match result.as_str() {
                    "ok" => SingleTestStatus::Passed,
                    _ => SingleTestStatus::Failed
                };
    
                let path = Path::new(OsStr::new(""));
                return Some(SingleTest::new(test.to_string(), status, path, 0))
            };
        }

        None
    }
}

fn split_and_trim(line: &str) -> Option<(String, String)> {
    // Split the line into at most two parts by "..."
    let mut parts = line.splitn(2, "...");

    // Get the first and second parts, if they exist
    let first = parts.next()?.trim().to_string();  // Get and trim first part
    let second = parts.next()?.trim().to_string(); // Get and trim second part

    Some((first, second))
}
