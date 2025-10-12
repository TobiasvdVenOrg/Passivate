macro_rules! mod_use {
    ($module: ident) => {
        pub mod $module;

        // false positive
        #[allow(unused_imports)]
        pub use $module::*;
    };
}

mod_use!(app);
mod_use!(error_app);
mod_use!(startup_errors);

pub mod views
{
    mod_use!(view);
    mod_use!(test_run_view);
    mod_use!(coverage_view);
    mod_use!(log_view);
    mod_use!(configuration_view);
    mod_use!(details_view);
}

mod_use!(run);

#[cfg(test)]
pub mod tests
{
    mod_use!(test_run_view_tests);
    // mod_use!(coverage_view_tests);
    pub mod coverage_view_tests;
    pub use coverage_view_tests::*;
    mod_use!(log_view_tests);
    mod_use!(configuration_view_tests);
    mod_use!(details_view_tests);
}
