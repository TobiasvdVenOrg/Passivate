use std::thread;

use clap::Parser;
use passivate::start;
use passivate_core::compose::compose;
use passivate_core::passivate_args::PassivateArgs;
use passivate_core::startup_errors::StartupError;

fn main() -> Result<(), StartupError>
{
    let args = PassivateArgs::parse();

    thread::scope(|scope| {
        let passivate = compose(args, scope)?;
        start::run_app(passivate)
    })
}
