use crate::{assert_matches, passivate_cargo::parse_status, test_execution::TestsStatus};

const SIMPLE_TEST_OUTPUT: &str = r#"
      Timing report saved to F:\Projects\Passivate\test_data\change_event_causes_test_run_and_results\target\cargo-timings\cargo-timing-20250213T224933Z.html
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src\lib.rs (target\x86_64-pc-windows-msvc\debug\deps\sample_project-6f071feeb7e729fe.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\add_tests.rs (target\x86_64-pc-windows-msvc\debug\deps\add_tests-033c785e73b09f11.exe)

running 2 tests
test add_2_and_2_is_4 ... ok
test add_2_and_4_is_6 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\multiply_tests.rs (target\x86_64-pc-windows-msvc\debug\deps\multiply_tests-7a0bb54228b46453.exe)

running 1 test
test multiply_2_and_2_is_4 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
"#;

#[test]
pub fn simple_test_output_includes_3_completed_tests() {        
    let status = parse_status(SIMPLE_TEST_OUTPUT);

    let completed = assert_matches!(status, TestsStatus::Completed);
    assert_eq!(3, completed.tests.len());
}

#[test]
pub fn test_with_error_in_its_name_is_not_considered_a_build_failure() {
    let test_output = r#"
    test some_test_with_error ... ok
    "#;

    let status = parse_status(test_output);
    assert_matches!(status, TestsStatus::Completed);
}

#[test]
pub fn test_with_error_in_its_name_is_not_considered_a_build_failure2() {
    let test_output = r#"
    ---- tests::test_execution::test_runner_tests::when_test_run_fails_error_is_reported stdout ----
    "#;

    let status = parse_status(test_output);
    assert_matches!(status, TestsStatus::Completed);
}