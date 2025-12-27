use crate::hyp_state::HypState;

pub trait Evaluate
{
    fn state(&self) -> HypState;
}
