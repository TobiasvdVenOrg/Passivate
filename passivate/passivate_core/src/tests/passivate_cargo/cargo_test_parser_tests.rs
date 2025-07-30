use crate::assert_matches;
use crate::passivate_cargo::CargoTestParser;
use crate::test_execution::ParseOutput;
use crate::test_run_model::{SingleTestStatus, TestRunEvent};

#[test]
pub fn parse_test_line_status_pass()
{
    let mut parser = CargoTestParser;
    let event = parser.parse_line("test add_2_and_2_is_4 ... ok").unwrap();

    let test = assert_matches!(event, TestRunEvent::TestFinished);
    assert_eq!("add_2_and_2_is_4", test.name);
    assert!(matches!(test.status, SingleTestStatus::Passed));
}

#[test]
pub fn test_with_error_in_its_name_is_not_considered_a_build_failure()
{
    let mut parser = CargoTestParser;
    let event = parser.parse_line("test some_test_with_error ... ok").unwrap();

    let test = assert_matches!(event, TestRunEvent::TestFinished);
    assert_eq!("some_test_with_error", test.name);
    assert!(matches!(test.status, SingleTestStatus::Passed));
}
