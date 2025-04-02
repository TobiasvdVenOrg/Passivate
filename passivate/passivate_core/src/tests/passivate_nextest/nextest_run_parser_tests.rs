
use galvanic_assert::{assert_that, matchers::*, structure};

use crate::{passivate_nextest::NextestParser, test_execution::ParseOutput, test_run_model::{SingleTest, SingleTestStatus, TestRunEvent}};

#[test]
pub fn parse_test_line_status_pass() {
    let mut parser = NextestParser::default(); 
    let line = "    PASS [   0.015s] sample_project::add_tests add_2_and_4_is_6";
    let event = parser.parse_line(line).unwrap();

    assert_that!(&event, structure!(TestRunEvent::TestFinished [
        eq(SingleTest::new("add_2_and_4_is_6".to_string(), SingleTestStatus::Passed, vec![]))
    ]));
}

#[test]
pub fn parse_test_line_status_fail() {
    let line = "        FAIL [   0.139s] passivate tests::coverage_view_tests::when_grcov_is_not_installed_error_is_reported";
    let event = NextestParser::default().parse_line(line).unwrap();

    assert_that!(&event, structure!(TestRunEvent::TestFinished [
        eq(SingleTest::new("when_grcov_is_not_installed_error_is_reported".to_string(), SingleTestStatus::Failed, vec![]))
    ]));
}
