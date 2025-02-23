use passivate::{run, StartupError};

fn main() -> Result<(), StartupError> {
    run(Box::new(|_context| { }))
}