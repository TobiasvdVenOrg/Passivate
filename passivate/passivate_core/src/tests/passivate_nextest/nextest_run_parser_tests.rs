use crate::{passivate_nextest::parse_line, test_execution::SingleTestStatus};

#[test]
pub fn parse_test_line_status_pass() {        
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let status = parse_line(line).expect("Expected single test status");

    assert!(matches!(status.status, SingleTestStatus::Passed));
}

#[test]
pub fn parse_test_line_status_name() {        
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let status = parse_line(line).expect("Expected single test status");

    assert_eq!("add_2_and_4_is_6", status.name);
}
