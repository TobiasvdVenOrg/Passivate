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
}

pub mod cross_cutting {
    mod_use!(log_event);
    mod_use!(log);
}

pub mod test_execution {
    mod_use!(change_event_handler);
    mod_use!(run_tests);
    mod_use!(parse_output);
    mod_use!(test_runner);
    mod_use!(test_run_iterator);
    mod_use!(test_run_processor);
    mod_use!(test_run_errors);
    mod_use!(test_run_handler);
}

pub mod test_run_model {
    mod_use!(single_test);
    mod_use!(single_test_status);
    mod_use!(test_run);
    mod_use!(test_run_events);
    mod_use!(snapshots);
    mod_use!(test_collection);
    mod_use!(test_id);
}

pub mod passivate_cargo {
    mod_use!(cargo_test_parser);
    mod_use!(cargo_workspace);
    mod_use!(cargo_workspace_errors);
}

pub mod passivate_nextest {
    mod_use!(nextest_run_parser);
}

pub mod passivate_grcov {
    mod_use!(covdir_json);
    mod_use!(grcov);
}

pub mod coverage {
    mod_use!(coverage_status);
    mod_use!(compute_coverage);
    mod_use!(coverage_errors);
}

pub mod test_helpers {
    pub mod assert_matches;
    pub mod builder;

    pub mod fakes {
        pub mod change_event_handler_fakes;
        pub mod channel_fakes;
        pub mod test_run_handler_fakes;
    }
}

pub mod configuration {
    mod_use!(passivate_config);
    mod_use!(test_runner_implementation);
    mod_use!(configuration_events);
    mod_use!(configuration_handler);
}

pub mod delegation {
    mod_use!(actor);
    mod_use!(actor_api);
    mod_use!(handler);
    mod_use!(actor_event);
    mod_use!(give);
    mod_use!(loan);
}

#[cfg(test)]
pub mod tests {
    mod actors {
        mod actor_tests;
    }

    mod configuration {
        mod configuration_handler_tests;
    }

    mod passivate_grcov {
        mod covdir_json_tests;
        mod grcov_tests;
    }
    
    mod passivate_cargo {
        mod cargo_test_parser_tests;
        mod cargo_workspace_tests;
    }

    mod passivate_nextest {
        mod nextest_run_parser_tests;
    }
    
    mod test_execution {
        mod test_run_handler_tests;
        mod test_run_processor_tests;
    }
}
