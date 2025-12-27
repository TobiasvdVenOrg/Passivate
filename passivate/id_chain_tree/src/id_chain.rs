use crate::tree::ChainLink;

pub trait IdChain
{
    type Link: ChainLink;

    fn chain(&self) -> &[Self::Link];
}

pub trait Depth
{
    fn depth(&self) -> usize;
}

impl<T> Depth for T
where
    T: IdChain
{
    fn depth(&self) -> usize
    {
        self.chain().len() - 1
    }
}
