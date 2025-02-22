mod app;
mod error_app;
mod startup_errors;
mod views;
mod passivate_notify;
mod run;

use run::run;

fn main() {
    run();
}