#[macro_export]
macro_rules! chain {
    ( $( $e:expr ),* ) => {
        {
            &[$($e.into()),*]
        }
    };
}
