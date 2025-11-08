use std::borrow::Borrow;
use std::collections::HashSet;
use std::collections::hash_set::Iter;

use passivate_hyp_names::hyp_id::HypId;

use crate::hyp::Hyp;
use crate::hyp_state::HypState;

pub struct HypSet<'a>
{
    hyps: HashSet<&'a Hyp>
}

impl<'a> HypSet<'a>
{
    pub fn new(hyps: HashSet<&'a Hyp>) -> Self
    {
        Self { hyps }
    }

    pub fn iter(&self) -> HypIterator<'_>
    {
        HypIterator { iter: self.hyps.iter() }
    }

    pub fn by_id(&self, id: &HypId) -> Option<&'a Hyp>
    {
        self.hyps.iter().find(|h| h.id == *id).copied()
    }

    pub fn current_state(&self) -> HypState
    {
        if self.iter().all(|h| h.current_state() == HypState::Passed)
        {
            HypState::Passed
        }
        else
        {
            HypState::Failed
        }
    }
}

#[derive(Debug)]
pub struct HypIterator<'a>
{
    iter: Iter<'a, &'a Hyp>
}

impl<'a> Iterator for HypIterator<'a>
{
    type Item = &'a Hyp;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter.next().copied()
    }
}

#[cfg(test)]
pub mod tests
{
    use std::collections::HashSet;

    use assert_matches::assert_matches;
    use passivate_hyp_names::hyp_id::HypId;

    use crate::hyp::Hyp;
    use crate::hyp_set::HypSet;
    use crate::hyp_state::HypState;

    #[test]
    pub fn find_hyp_by_id()
    {
        let hyp1_id = HypId::new("crate", "hyp1").unwrap();

        let hyp1 = Hyp::new(hyp1_id.clone(), HypState::Passed);
        let hyp2 = Hyp::new(HypId::new("crate", "hyp2").unwrap(), HypState::Passed);

        let collection = HypSet::new(HashSet::from_iter([&hyp1, &hyp2]));

        let found = collection.by_id(&hyp1_id);

        assert_eq!(found, Some(&hyp1));
    }

    #[test]
    pub fn iterate_hyp_collection()
    {
        let hyp1_id = HypId::new("crate", "hyp1").unwrap();

        let hyp1 = Hyp::new(hyp1_id.clone(), HypState::Passed);
        let hyp2 = Hyp::new(HypId::new("crate", "hyp2").unwrap(), HypState::Passed);

        let collection = HypSet::new(HashSet::from_iter([&hyp1, &hyp2]));

        let collected: Vec<&Hyp> = collection.iter().collect();

        assert!(collected.contains(&&hyp1));
        assert!(collected.contains(&&hyp2));
    }

    #[test]
    pub fn collection_state_is_passed_when_all_hyps_passed()
    {
        let hyp1_id = HypId::new("crate", "hyp1").unwrap();

        let hyp1 = Hyp::new(hyp1_id.clone(), HypState::Passed);
        let hyp2 = Hyp::new(HypId::new("crate", "hyp2").unwrap(), HypState::Passed);

        let collection = HypSet::new(HashSet::from_iter([&hyp1, &hyp2]));

        assert_matches!(collection.current_state(), HypState::Passed);
    }
}
