use crate::test_run_model::TestRunEvent;

pub trait ParseOutput {
    fn parse_line(&self, line: &str) -> Option<TestRunEvent>;
}
