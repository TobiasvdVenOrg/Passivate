use core::slice;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

use hashbrown::DefaultHashBuilder;

pub trait HypTreeValue<TPart: Hash>
{
    fn path(&self) -> &[TPart];
}

fn hash_path<TPart: Hash, H: Hasher>(value: impl HypTreeValue<TPart>, state: &mut H)
{
    for part in value.path()
    {
        part.hash(state);
    }
}

pub struct HypNode<TValue>
{
    value: TValue
}

pub struct HypTree<TPart: Hash, TValue: HypTreeValue<TPart>, TBuildHasher = DefaultHashBuilder>
{
    values: Vec<HypNode<TValue>>,
    build_hasher: TBuildHasher,
    _phantom_tpart: PhantomData<TPart>
}

pub struct Hyperator<'a, TValue>
{
    iter: slice::Iter<'a, HypNode<TValue>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth(u16);

impl Depth
{
    pub fn new(depth: u16) -> Self
    {
        Self(depth)
    }
}

impl<'a, TValue> Iterator for Hyperator<'a, TValue>
{
    type Item = (Depth, &'a TValue);

    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter.next().map(|node| (Depth(0), &node.value))
    }
}

impl<TPart: Hash, TValue: HypTreeValue<TPart>> HypTree<TPart, TValue, DefaultHashBuilder>
{
    pub fn new() -> Self
    {
        Self {
            values: Vec::new(),
            build_hasher: DefaultHashBuilder::default(),
            _phantom_tpart: PhantomData
        }
    }

    pub fn insert(&mut self, element: TValue)
    {
        self.values.push(HypNode { value: element });
    }
}

impl<TPart: Hash, TValue: HypTreeValue<TPart>, THasherBuilder: BuildHasher> HypTree<TPart, TValue, THasherBuilder>
{
    pub fn iter<'a>(&'a self) -> Hyperator<'a, TValue>
    {
        let vec_iterator = self.values.iter();

        Hyperator { iter: vec_iterator }
    }
}
