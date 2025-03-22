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
}

pub mod test_run_model {
    mod_use!(single_test);
    mod_use!(single_test_status);
    mod_use!(test_run);
    mod_use!(test_run_events);
}

pub mod passivate_cargo {
    mod_use!(cargo_test_parser);
}

pub mod passivate_nextest {
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

    pub mod fakes {
        mod_use!(change_event_handler_fakes);
        mod_use!(channel_fakes);
    }
}

pub mod configuration {
    mod_use!(passivate_config);
    mod_use!(test_runner_implementation);
    mod_use!(configuration_events);
    mod_use!(configuration_handler);
}

pub mod actors {
    mod_use!(actor);
    mod_use!(actor_api);
    mod_use!(handler);
    mod actor_event;
}

#[cfg(test)]
pub mod tests {
    mod actors {
        mod actor_tests;
    }

    mod configuration {
        mod configuration_handler_tests;
    }

    mod passivate_cargo {
        mod cargo_test_parser_tests;
    }

    mod passivate_nextest {
        mod nextest_run_parser_tests;
    }
    
    mod test_execution {
        mod change_event_handler_tests;
        mod test_run_processor_tests;
    }
}
