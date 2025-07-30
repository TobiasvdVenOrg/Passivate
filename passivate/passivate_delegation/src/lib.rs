macro_rules! mod_use {
    ($module: ident) => {
        pub mod $module;

        // false positive
        #[allow(unused_imports)]
        pub use $module::*;
    };
}

mod_use!(actor);
mod_use!(actor_tx);
mod_use!(handler);
mod_use!(actor_event);
mod_use!(tx_rx);

#[cfg(test)]
#[path = "actor.tests.rs"]
pub mod actor_tests;
