use crate::bridge::Bridge;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputReport<TBridge: Bridge>
{
    id: TBridge::Id,
    output: TBridge::Output
}

impl<TBridge: Bridge> OutputReport<TBridge>
{
    pub fn new(id: TBridge::Id, output: TBridge::Output) -> OutputReport<TBridge>
    {
        Self { id, output }
    }

    pub fn id(&self) -> &TBridge::Id
    {
        &self.id
    }
}
