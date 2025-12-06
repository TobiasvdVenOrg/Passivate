use std::fmt::Display;

use crate::bridge::{Bridge, HypPath};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hyp<TBridge: Bridge>
{
    pub info: TBridge::THypInfo
}

impl<TBridge: Bridge> Hyp<TBridge> {}

impl<TBridge: Bridge> HypPath for Hyp<TBridge>
{
    type TId = TBridge::TId;

    fn path(&self) -> Self::TId
    {
        self.info.path()
    }
}

impl<TBridge: Bridge> Display for Hyp<TBridge>
where
    TBridge::TId: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.path().fmt(f)
    }
}
