use passivate_hyp_names::hyp_id::HypId;

use crate::hyp::Hyp;
use crate::hyp_state::HypState;

pub trait HypIterator<'a>: Iterator<Item = &'a &'a Hyp> + Sized
{
    fn by_id(&mut self, id: &HypId) -> Option<&'a Hyp>
    {
        self.find(|h| h.id == *id).copied()
    }

    fn current_state(&mut self) -> HypState
    {
        if self.all(|h| h.current_state() == HypState::Passed)
        {
            HypState::Passed
        }
        else
        {
            HypState::Failed
        }
    }
}

pub struct HypIter;

impl<'a, T> HypIterator<'a> for T where T: Iterator<Item = &'a &'a Hyp> {}

#[cfg(test)]
pub mod tests
{
    use assert_matches::assert_matches;
    use passivate_hyp_names::hyp_id::HypId;

    use crate::hyp::Hyp;
    use crate::hyp_iter_ext::HypIterator;
    use crate::hyp_state::HypState;

    #[test]
    pub fn find_hyp_by_id()
    {
        let hyp1_id = HypId::new("crate", "hyp1").unwrap();

        let hyp1 = Hyp::new(hyp1_id.clone(), HypState::Passed);
        let hyp2 = Hyp::new(HypId::new("crate", "hyp2").unwrap(), HypState::Passed);

        let hyps = vec![&hyp1, &hyp2];
        let found = hyps.iter().by_id(&hyp1_id);

        assert_eq!(found, Some(&hyp1));
    }

    #[test]
    pub fn collection_state_is_passed_when_all_hyps_passed()
    {
        let hyp1_id = HypId::new("crate", "hyp1").unwrap();

        let hyp1 = Hyp::new(hyp1_id.clone(), HypState::Passed);
        let hyp2 = Hyp::new(HypId::new("crate", "hyp2").unwrap(), HypState::Passed);

        let hyps = vec![&hyp1, &hyp2];

        assert_matches!(hyps.iter().current_state(), HypState::Passed);
    }
}
