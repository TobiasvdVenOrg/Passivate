use core::slice;
use std::marker::PhantomData;

use crate::depth::Depth;
use crate::hyp_node::HypNode;
use crate::hyp_tree_value::HypTreeValue;

pub struct HypTree<TPart: Eq, TValue: HypTreeValue<Part = TPart>>
{
    values: Vec<HypNode<TValue>>,
    _phantom_tpart: PhantomData<TPart>
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

pub struct Hyperator<'a, TValue>
{
    iter: slice::Iter<'a, HypNode<TValue>>
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
