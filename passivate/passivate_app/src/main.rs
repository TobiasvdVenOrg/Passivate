use clap::Parser;
use passivate_core::{passivate_args::PassivateArgs, compose::compose, startup_errors::StartupError};
use passivate::run::run_app;

fn main() -> Result<(), StartupError>
{
    let args = PassivateArgs::parse();
    compose(args, run_app)
}
