use std::borrow::Cow;

use crate::hyp_state::HypState;

pub trait BridgeHyp
{
    type Id;

    fn id(&self) -> &Self::Id;
    fn name(&self) -> Cow<'_, str>;
    fn state(&self) -> Option<HypState>;
}
