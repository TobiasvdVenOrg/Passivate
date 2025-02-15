
#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:pat $( if $guard:expr )?) => {
        match &$value {
            $pattern $( if $guard )? => (),
            _ => panic!(
                "assertion failed: expected `{}` to match `{}`",
                stringify!($value),
                stringify!($pattern)
            ),
        }
    };
}