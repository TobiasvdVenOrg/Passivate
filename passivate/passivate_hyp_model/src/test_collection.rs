use core::slice;
use std::slice::{Iter, IterMut};

use itertools::Itertools;
use passivate_hyp_names::hyp_id::HypId;

use crate::single_hyp::SingleHyp;

#[derive(Default, Clone, Debug)]
pub struct TestCollection
{
    tests: Vec<SingleHyp>
}

impl TestCollection
{
    pub fn add(&mut self, test: SingleHyp)
    {
        self.tests.push(test);
    }

    pub fn add_or_update(&mut self, test: SingleHyp)
    {
        match self.tests.iter_mut().find(|t| t.id == test.id)
        {
            Some(existing) => *existing = test,
            None => self.add(test)
        };
    }

    pub fn find(&self, id: &HypId) -> Option<&SingleHyp>
    {
        self.tests.iter().find(|t| t.id == *id)
    }

    pub fn find_mut(&mut self, id: &HypId) -> Option<&mut SingleHyp>
    {
        self.tests.iter_mut().find(|t| t.id == *id)
    }

    pub fn clear(&mut self)
    {
        self.tests.clear();
    }

    pub fn clear_except(&mut self, id: &HypId) -> Option<&mut SingleHyp>
    {
        self.tests.retain(|h| h.id == *id);
        self.tests.iter_mut().exactly_one().ok()
    }

    pub fn is_empty(&self) -> bool
    {
        self.tests.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, SingleHyp>
    {
        self.tests.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, SingleHyp>
    {
        self.tests.iter_mut()
    }
}

impl IntoIterator for TestCollection
{
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = SingleHyp;

    fn into_iter(self) -> Self::IntoIter
    {
        self.tests.into_iter()
    }
}

impl<'a> IntoIterator for &'a TestCollection
{
    type IntoIter = slice::Iter<'a, SingleHyp>;
    type Item = &'a SingleHyp;

    fn into_iter(self) -> Self::IntoIter
    {
        self.tests.iter()
    }
}

impl<'a> IntoIterator for &'a mut TestCollection
{
    type IntoIter = slice::IterMut<'a, SingleHyp>;
    type Item = &'a mut SingleHyp;

    fn into_iter(self) -> Self::IntoIter
    {
        self.tests.iter_mut()
    }
}
