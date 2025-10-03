macro_rules! mod_use {
    ($module: ident) => {
        pub mod $module;

        // false positive
        #[allow(unused_imports)]
        pub use $module::*;
    };
}

pub mod change_events
{
    mod_use!(change_event);
}

pub mod cross_cutting
{
    mod_use!(log_event);
}

pub mod test_execution
{
    mod_use!(change_event_handler);
    mod_use!(test_runner);
    mod_use!(test_run_iterator);
    mod_use!(test_run_errors);
    mod_use!(test_run_handler);
}

pub mod test_run_model
{
    mod_use!(single_test);
    mod_use!(single_test_status);
    mod_use!(test_run);
    mod_use!(test_run_events);
    mod_use!(snapshots);
    mod_use!(test_collection);
    mod_use!(test_id);
}

pub mod passivate_cargo
{
    mod_use!(cargo_workspace);
    mod_use!(cargo_workspace_errors);
}

pub mod passivate_nextest
{
    mod_use!(nextest_cargo_options);
}

pub mod passivate_grcov
{
    mod_use!(covdir_json);
    mod_use!(grcov);
}

pub mod coverage
{
    mod_use!(coverage_status);
    mod_use!(compute_coverage);
    mod_use!(coverage_errors);
}

pub mod configuration
{
    mod_use!(passivate_config);
    mod_use!(configuration_event);
    mod_use!(configuration_manager);
}

pub mod test_helpers
{
    pub mod assert_matches;
    pub mod test_run_setup;
    pub mod test_name;
}

#[cfg(test)]
pub mod tests
{
    mod configuration
    {
        mod configuration_manager_tests;
    }

    mod passivate_grcov
    {
        mod covdir_json_tests;
        mod grcov_tests;
    }

    mod passivate_cargo
    {
        mod cargo_workspace_tests;
    }

    mod test_execution
    {
        mod test_run_handler_tests;
    }
}
