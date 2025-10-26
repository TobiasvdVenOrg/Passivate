use passivate_core::passivate_state::PassivateState;

use crate::details_view::HypDetails;

#[derive(Default)]
pub struct PassivateViewState
{
    pub hyp_details: Option<HypDetails>
}

impl PassivateViewState
{
    pub fn update(old: &PassivateState, new: &PassivateState)
    {

    }
}