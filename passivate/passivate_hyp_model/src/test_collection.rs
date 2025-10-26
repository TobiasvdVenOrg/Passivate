use core::slice;

use itertools::Itertools;
use passivate_hyp_names::hyp_id::HypId;

use crate::single_test::SingleTest;

#[derive(Default, Clone, Debug)]
pub struct TestCollection
{
    tests: Vec<SingleTest>
}

impl TestCollection
{
    pub fn add(&mut self, test: SingleTest)
    {
        self.tests.push(test);
    }

    pub fn add_or_update(&mut self, test: SingleTest)
    {
        match self.tests.iter_mut().find(|t| t.id == test.id)
        {
            Some(existing) => *existing = test,
            None => self.add(test)
        };
    }

    pub fn find(&self, id: &HypId) -> Option<&SingleTest>
    {
        self.tests.iter().find(|t| t.id == *id)
    }

    pub fn find_mut(&mut self, id: &HypId) -> Option<&mut SingleTest>
    {
        self.tests.iter_mut().find(|t| t.id == *id)
    }

    pub fn clear(&mut self)
    {
        self.tests.clear();
    }

    pub fn clear_except(&mut self, id: &HypId) -> Option<&mut SingleTest>
    {
        self.tests.retain(|h| h.id == *id);
        self.tests.iter_mut().exactly_one().map_or(None, |h| Some(h))
    }

    pub fn is_empty(&self) -> bool
    {
        self.tests.is_empty()
    }
}

impl IntoIterator for TestCollection
{
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = SingleTest;

    fn into_iter(self) -> Self::IntoIter
    {
        self.tests.into_iter()
    }
}

impl<'a> IntoIterator for &'a TestCollection
{
    type IntoIter = slice::Iter<'a, SingleTest>;
    type Item = &'a SingleTest;

    fn into_iter(self) -> Self::IntoIter
    {
        self.tests.iter()
    }
}

impl<'a> IntoIterator for &'a mut TestCollection
{
    type IntoIter = slice::IterMut<'a, SingleTest>;
    type Item = &'a mut SingleTest;

    fn into_iter(self) -> Self::IntoIter
    {
        self.tests.iter_mut()
    }
}
