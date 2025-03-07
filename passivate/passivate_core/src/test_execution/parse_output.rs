use super::SingleTest;

pub trait ParseOutput {
    fn parse_line(&self, line: &str) -> Option<SingleTest>;
}
