use super::SingleTestStatus;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct SingleTest {
    pub name: String,
    pub status: SingleTestStatus
}
