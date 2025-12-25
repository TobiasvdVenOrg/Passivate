pub trait HypTreeValue
{
    type Part;

    fn path(&self) -> &[Self::Part];
}
