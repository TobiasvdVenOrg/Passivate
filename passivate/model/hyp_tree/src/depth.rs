#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(usize);

impl Depth
{
    pub fn new(depth: usize) -> Self
    {
        Self(depth)
    }
}
