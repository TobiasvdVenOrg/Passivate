macro_rules! mod_use {
    ($module: ident) => {
        pub mod $module;

        // false positive
        #[allow(unused_imports)]
        pub use $module::*;
    }
}

pub mod change_events {
    mod_use!(change_event);
    mod_use!(handle_change_event);
}

pub mod test_execution {
    mod_use!(single_test);
    mod_use!(single_test_status);
    mod_use!(test_runner);
    mod_use!(tests_status);
    mod_use!(run_tests);
    mod_use!(run_tests_errors);
}

pub mod passivate_cargo {
    mod_use!(cargo_test);
    mod_use!(cargo_test_parser);
}

pub mod passivate_nextest {
    mod_use!(nextest_run);
    mod_use!(nextest_run_parser);
}

pub mod passivate_grcov {
    mod_use!(grcov);
}

pub mod coverage {
    mod_use!(coverage_status);
    mod_use!(compute_coverage);
    mod_use!(coverage_errors);
}

pub mod test_helpers {
    pub mod assert_matches;
}

pub mod dispatching {
    mod_use!(dispatch);
    mod_use!(dispatch_error);
    mod_use!(sender_dispatch);
}

#[cfg(test)]
pub mod tests {
    mod passivate_cargo {
        mod cargo_test_parser_tests;
    }

    mod passivate_nextest {
        mod nextest_run_parser_tests;
    }
    
    mod test_execution {
        mod test_runner_tests;
    }
}
