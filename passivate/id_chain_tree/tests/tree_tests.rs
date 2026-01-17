use itertools::assert_equal;
use passivate_id_chain_tree::id_chain::{Depth, IdChain};

#[macro_use]
extern crate assert_matches;

#[macro_use]
extern crate passivate_id_chain_tree;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestValue
{
    name: String,
    path: Vec<String>
}

impl TestValue
{
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self
    {
        Self {
            name: name.into(),
            path: path.into().split("::").map(String::from).collect()
        }
    }

    pub fn change_name(&mut self, new_name: impl Into<String>)
    {
        self.name = new_name.into();
    }
}

impl IdChain for TestValue
{
    type Link = String;

    fn chain(&self) -> &[Self::Link]
    {
        self.path.as_slice()
    }
}

#[test]
pub fn child_node_has_greater_depth_than_parent()
{
    let parent = TestValue::new("PARENT", "A");
    let child = TestValue::new("CHILD", "A::B");

    let tree = tree!(parent, child);

    let mut iter = tree.iter();

    assert_matches!(iter.next(), Some(value) => {
        assert_eq!(0, value.depth());
        assert_eq!("PARENT", value.name);
    });

    assert_matches!(iter.next(), Some(value) => {
        assert_eq!(1, value.depth());
        assert_eq!("CHILD", value.name);
    });
}

#[test]
pub fn find_value_by_chain()
{
    let tree = tree!(TestValue::new("FIND_THIS", "some::chain::id"));

    let chain = chain!("some", "chain", "id");

    assert_matches!(tree.get(chain).unwrap(), value => {
        assert_eq!("FIND_THIS", value.name);
    });
}

#[test]
pub fn find_and_mutate_value()
{
    let mut tree = tree!(TestValue::new("ORIGINAL_NAME", "some::chain::id"));

    let chain = chain!("some", "chain", "id");

    let value = tree.get_mut(chain).unwrap();
    value.change_name("MODIFIED");

    assert_matches!(tree.get(chain).unwrap(), value => {
        assert_eq!("MODIFIED", value.name);
    });
}

#[test]
pub fn iterate_children_of_a_node()
{
    let parent = TestValue::new("PARENT", "A");
    let child1 = TestValue::new("CHILD1", "A::1");
    let child2 = TestValue::new("CHILD2", "A::2");

    let tree = tree!(parent, child1.clone(), child2.clone());

    let parent_chain = chain!("A");
    let node = tree.get_node(parent_chain).unwrap();

    assert_equal(node.iter_children(), [&child1, &child2]);
}
