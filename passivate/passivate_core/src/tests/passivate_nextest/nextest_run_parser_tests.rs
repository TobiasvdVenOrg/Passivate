use crate::{assert_matches, passivate_nextest::NextestParser, test_execution::ParseOutput, test_run_model::{SingleTestStatus, TestRunEvent}};

#[test]
pub fn parse_test_line_status_pass() {
    let parser = NextestParser; 
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let event = parser.parse_line(line).unwrap();

    let test = assert_matches!(event, TestRunEvent::TestFinished);
    assert!(matches!(test.status, SingleTestStatus::Passed));
}

#[test]
pub fn parse_test_line_status_name() {
    let parser = NextestParser;      
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let event = parser.parse_line(line).unwrap();

    let test = assert_matches!(event, TestRunEvent::TestFinished);
    assert_eq!("add_2_and_4_is_6", test.name);
}

#[test]
pub fn parse_test_line_status_fail() {
    let line = "        FAIL [   0.139s] passivate tests::coverage_view_tests::when_grcov_is_not_installed_error_is_reported";
    let event = NextestParser.parse_line(line).unwrap();

    let test = assert_matches!(event, TestRunEvent::TestFinished);
    assert!(matches!(test.status, SingleTestStatus::Failed));
}