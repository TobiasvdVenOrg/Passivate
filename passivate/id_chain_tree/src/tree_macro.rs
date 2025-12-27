#[macro_export]
macro_rules! tree {
    () => {
        $crate::tree::Tree::new()
    };
    ( $( $e:expr ),* ) => {
        {
            let mut new_tree = $crate::tree::Tree::new();

            $(
                new_tree.insert($e);
            )*

            new_tree
        }
    };
}
