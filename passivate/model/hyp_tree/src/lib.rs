use core::slice;
use std::marker::PhantomData;

pub trait HypTreeValue
{
    type Part;

    fn path(&self) -> &[Self::Part];
}

pub struct HypNode<TValue>
{
    value: TValue
}

impl<TPart, TValue> HypNode<TValue>
where
    TPart: Eq,
    TValue: HypTreeValue<Part = TPart>
{
    pub fn depth(&self) -> Depth
    {
        Depth(self.value.path().len() - 1)
    }
}

pub struct HypTree<TPart: Eq, TValue: HypTreeValue<Part = TPart>>
{
    values: Vec<HypNode<TValue>>,
    _phantom_tpart: PhantomData<TPart>
}

pub struct Hyperator<'a, TValue>
{
    iter: slice::Iter<'a, HypNode<TValue>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(usize);

impl Depth
{
    pub fn new(depth: usize) -> Self
    {
        Self(depth)
    }
}

impl<'a, TPart, TValue> Iterator for Hyperator<'a, TValue>
where
    TPart: Eq,
    TValue: HypTreeValue<Part = TPart>
{
    type Item = (Depth, &'a TValue);

    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter.next().map(|node| (node.depth(), &node.value))
    }
}

impl<TPart: Eq, TValue: HypTreeValue<Part = TPart>> HypTree<TPart, TValue>
{
    pub fn new() -> Self
    {
        Self {
            values: Vec::new(),
            _phantom_tpart: PhantomData
        }
    }

    pub fn insert(&mut self, element: TValue)
    {
        self.values.push(HypNode { value: element });
    }

    pub fn iter<'a>(&'a self) -> Hyperator<'a, TValue>
    {
        let vec_iterator = self.values.iter();

        Hyperator { iter: vec_iterator }
    }
}
