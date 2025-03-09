use crate::test_run_model::SingleTest;

pub trait ParseOutput {
    fn parse_line(&self, line: &str) -> Option<SingleTest>;
}
