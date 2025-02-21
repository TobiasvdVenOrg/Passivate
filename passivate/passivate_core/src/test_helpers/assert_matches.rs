
#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:path) => {
        match $value {
            $pattern(result) => result, 
            _ => panic!(
                "assertion failed: expected `{}` to match `{}`",
                stringify!($value),
                stringify!($pattern)
            ),
        }
    };
}
