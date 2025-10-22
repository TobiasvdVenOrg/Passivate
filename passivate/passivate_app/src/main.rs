use clap::Parser;
use passivate_core::{passivate_args::PassivateArgs, run::run, startup_errors::StartupError};
use passivate::run::run_app;

fn main() -> Result<(), StartupError>
{
    let args = PassivateArgs::parse();
    run(args, run_app)
}
