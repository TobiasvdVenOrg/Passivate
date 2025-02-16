use std::{ffi::OsStr, path::Path};
use crate::test_execution::{SingleTest, SingleTestStatus, TestsStatus};

pub fn parse_status(text: &str) -> TestsStatus {
    let mut tests = Vec::new();

    for line in text.lines() {
        let line = line.trim();

        if line.starts_with("test") {
            if let Some((test, result)) = split_and_trim(line) {
                let status = match result.as_str() {
                    "ok" => SingleTestStatus::Passed,
                    _ => SingleTestStatus::Failed
                };
    
                let path = Path::new(OsStr::new(""));
                tests.push(SingleTest::new(test.to_string(), status, path, 0));
            }
        }
        else if line.contains("error") {
            return TestsStatus::build_failure(line)
        }
    }

    TestsStatus::completed(tests)
}

fn split_and_trim(line: &str) -> Option<(String, String)> {
    // Split the line into at most two parts by "..."
    let mut parts = line.splitn(2, "...");

    // Get the first and second parts, if they exist
    let first = parts.next()?.trim().to_string();  // Get and trim first part
    let second = parts.next()?.trim().to_string(); // Get and trim second part

    Some((first, second))
}
