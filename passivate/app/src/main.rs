use clap::Parser;
use passivate::start;
use passivate_core::compose::compose;
use passivate_core::passivate_args::PassivateArgs;
use passivate_core::startup_errors::StartupError;
use passivate_run_rust::hyp_run_handler;

fn main() -> Result<(), StartupError>
{
    let args = PassivateArgs::parse();

    let runtime = hyp_run_handler::build_tokio_runtime();
    let passivate = compose(args, &runtime)?;
    start::run_app(passivate)
}
