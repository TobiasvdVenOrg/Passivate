use passivate_core::passivate_state::PassivateState;

use crate::details_view::HypDetails;

#[derive(Default)]
pub struct PassivateViewState<'a>
{
    pub hyp_details: Option<HypDetails<'a>>
}

impl PassivateViewState<'_>
{
    pub fn update(old: &PassivateState, new: &PassivateState)
    {

    }
}