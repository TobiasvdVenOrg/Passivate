use passivate_model_hyp_tree::depth::Depth;
use passivate_model_hyp_tree::hyp_tree::HypTree;
use passivate_model_hyp_tree::hyp_tree_value::HypTreeValue;

#[macro_use]
extern crate assert_matches;

#[derive(Debug)]
struct TestValue
{
    value: String,
    path: Vec<String>
}

impl TestValue
{
    pub fn new(value: impl Into<String>, path: impl Into<String>) -> Self
    {
        Self {
            value: value.into(),
            path: path.into().split("::").map(String::from).collect()
        }
    }
}

impl HypTreeValue for TestValue
{
    type Part = String;

    fn path(&self) -> &[Self::Part]
    {
        self.path.as_slice()
    }
}

#[test]
pub fn child_node_has_greater_depth_than_parent()
{
    let mut tree = HypTree::new();

    let parent = TestValue::new("PARENT", "A");
    let child = TestValue::new("CHILD", "A::B");

    tree.insert(parent);
    tree.insert(child);

    let mut iter = tree.iter();

    assert_matches!(iter.next(), Some((depth, value)) => {
        assert_eq!(Depth::new(0), depth);
        assert_eq!("PARENT", value.value);
    });

    assert_matches!(iter.next(), Some((depth, value)) => {
        assert_eq!(Depth::new(1), depth);
        assert_eq!("CHILD", value.value);
    });
}
