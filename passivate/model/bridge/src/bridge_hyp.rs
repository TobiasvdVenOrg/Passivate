use std::fmt::Display;

pub trait BridgeHyp: Display
{
    type Id;

    fn id(&self) -> &Self::Id;
}
