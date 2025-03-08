macro_rules! mod_use {
    ($module: ident) => {
        pub mod $module;

        // false positive
        #[allow(unused_imports)]
        pub use $module::*;
    }
}

mod_use!(app);
mod_use!(error_app);
mod_use!(startup_errors);

pub mod views {
    mod_use!(view);
    mod_use!(test_run_view);
    mod_use!(coverage_view);
}

pub mod passivate_notify {
    mod_use!(notify_change_events);
    mod_use!(notify_change_events_errors);
}

mod_use!(run);

#[cfg(test)]
pub mod tests {
    mod_use!(test_run_view_tests);
    mod_use!(coverage_view_tests);
}
