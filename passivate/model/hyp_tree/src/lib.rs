use std::collections::HashSet;
use std::hash::Hash;

pub trait HypTreeValue<P: Hash>
{
    fn path(&self) -> &[P];
}

impl<TPart: Hash, T> Hash for T
where
    T: HypTreeValue<TPart>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        state.write(self.path());
    }
}

impl<P: Hash, T> HypTreeValue<P> for HypNode<P, T>
{
    fn path(&self) -> &[P]
    {
        todo!()
    }
}

pub struct HypNode<P, T>
{
    hyp: T,
    path: Vec<P>
}

pub struct HypTree<P: Hash, T: HypTreeValue<P>>
{
    tree: HashSet<HypNode<P, T>>
}

impl<P: Hash + Clone, T: HypTreeValue<P>> HypTree<P, T>
{
    pub fn new(root: T) -> Self
    {
        let r = HypNode {
            hyp: root,
            path: root.path().iter().map(|p| p.hash()).collect()
        };

        let p = r.path();

        Self { tree: vec![root] }
    }
}

#[cfg(test)]
mod tests
{
    use crate::{HypTree, HypTreeValue};

    struct TestValue
    {
        value: i32,
        path: Vec<String>
    }

    impl TestValue
    {
        pub fn new(value: i32, path: impl Into<String>) -> Self
        {
            Self {
                value,
                path: path.into().split("::").map(String::from).collect()
            }
        }
    }

    impl HypTreeValue<String> for TestValue
    {
        fn path(&self) -> &[String]
        {
            todo!()
        }
    }

    #[test]
    pub fn new_hyp_tree_has_root()
    {
        let root = TestValue::new(0, "");
        let tree = HypTree::new(root);
    }
}
