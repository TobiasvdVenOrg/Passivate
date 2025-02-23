use libtest_mimic::Arguments;
use libtest_mimic::Trial;
use passivate::run_from_path;
use tokio::time;
use std::path::Path;
use std::time::Duration;
use libtest_mimic::Failed;
use tokio::task;

#[tokio::main]
async fn main() {
    let mut args = Arguments::from_args();

    let tests = vec![
        Trial::test("start_and_exit_passivate", start_and_exit_passivate)
    ];

    args.test_threads = Some(1);
    libtest_mimic::run(&args, tests).exit();
}

pub fn start_and_exit_passivate() -> Result<(), Failed> {
    run_from_path(Path::new("..\\..\\test_data\\start_and_exit_passivate"), Box::new(move |context: egui::Context| {
        task::spawn(async move {   
            // Asynchronously send a close window command to passivate after some delay        
            time::sleep(Duration::from_secs(8)).await;
            context.send_viewport_cmd(egui::ViewportCommand::Close);
        });
    }))?;

    // This test does not assert, it exists to ensure that passivate starts (the context_accessor is invoked) and exits without hanging
    // The test does not pass in the case where a timeout occurs
    // The timeout is configured in .config/nextest.toml 
    Ok(())
}
