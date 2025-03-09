use super::SingleTestStatus;

#[derive(Clone)]
#[derive(Debug)]
pub struct SingleTest {
    pub name: String,
    pub status: SingleTestStatus
}
