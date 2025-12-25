use crate::depth::Depth;
use crate::hyp_tree_value::HypTreeValue;

pub struct HypNode<TValue>
{
    pub value: TValue
}

impl<TPart, TValue> HypNode<TValue>
where
    TPart: Eq,
    TValue: HypTreeValue<Part = TPart>
{
    pub fn depth(&self) -> Depth
    {
        Depth::new(self.value.path().len() - 1)
    }
}
