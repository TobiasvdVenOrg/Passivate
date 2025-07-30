use passivate::{StartupError, run};

fn main() -> Result<(), StartupError>
{
    run(Box::new(|_context| {}))
}
