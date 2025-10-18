use passivate::{run::run, startup_errors::StartupError};


fn main() -> Result<(), StartupError>
{
    run(Box::new(|_context| {}))
}
