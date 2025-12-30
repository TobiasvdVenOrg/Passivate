use passivate_model_bridge::bridge::Bridge;
use passivate_model_bridge::hyp_state::HypState;

use crate::hyp::Hyp;

pub trait Evaluate<TBridge: Bridge>
{
    fn state_of(&self, hyp: Hyp<TBridge>) -> HypState;
}
