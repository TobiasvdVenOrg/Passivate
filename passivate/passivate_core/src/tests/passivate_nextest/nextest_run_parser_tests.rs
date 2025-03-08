use crate::{passivate_nextest::NextestParser, test_execution::{ParseOutput, SingleTestStatus}};

#[test]
pub fn parse_test_line_status_pass() {
    let parser = NextestParser; 
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let status = parser.parse_line(line).expect("Expected single test status");

    assert!(matches!(status.status, SingleTestStatus::Passed));
}

#[test]
pub fn parse_test_line_status_name() {
    let parser = NextestParser;      
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let status = parser.parse_line(line).expect("Expected single test status");

    assert_eq!("add_2_and_4_is_6", status.name);
}

#[test]
pub fn parse_test_line_status_fail() {
    let status = NextestParser.parse_line("        FAIL [   0.139s] passivate tests::coverage_view_tests::when_grcov_is_not_installed_error_is_reported").unwrap();

    assert!(matches!(status.status, SingleTestStatus::Failed));
}