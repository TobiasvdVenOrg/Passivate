use crate::{passivate_cargo::CargoTestParser, test_execution::{ParseOutput, SingleTestStatus}};

#[test]
pub fn parse_test_line_status_pass() {
    let parser = CargoTestParser;
    let status = parser.parse_line("test add_2_and_2_is_4 ... ok").expect("Expected single test status");

    assert!(matches!(status.status, SingleTestStatus::Passed));
}

#[test]
pub fn test_with_error_in_its_name_is_not_considered_a_build_failure() {
    let parser = CargoTestParser;
    let status = parser.parse_line("test some_test_with_error ... ok").expect("Expected single test status");

    assert!(matches!(status.status, SingleTestStatus::Passed));
}
