use clap::Parser;
use passivate::start;
use passivate_core::compose::compose;
use passivate_core::passivate_args::PassivateArgs;
use passivate_core::startup_errors::StartupError;

fn main() -> Result<(), StartupError>
{
    let args = PassivateArgs::parse();
    let passivate = compose(args)?;
    start::run_app(passivate)
}
