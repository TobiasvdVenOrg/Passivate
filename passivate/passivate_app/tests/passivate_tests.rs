#[cfg(target_os = "windows")]
use libtest_mimic::Failed;

#[cfg(target_os = "windows")]
#[tokio::main]
async fn main() {
    use libtest_mimic::Arguments;
    use libtest_mimic::Trial;

    let mut args = Arguments::from_args();

    let tests = vec![
        Trial::test("start_and_exit_passivate", start_and_exit_passivate)
    ];

    args.test_threads = Some(1);

    // We use libtest-mimic because it allows us to run our tests on the main thread
    // This is a requirement for these tests, which actually start passivate
    libtest_mimic::run(&args, tests).exit();
}

#[cfg(target_os = "windows")]
pub fn start_and_exit_passivate() -> Result<(), Failed> {
    use passivate::run_from_path;
    use tokio::time;
    use std::path::Path;
    use std::time::Duration;
    use tokio::task;

    run_from_path(Path::new("..\\..\\test_data\\simple_project"), Box::new(move |context: egui::Context| {
        task::spawn(async move {   
            // Asynchronously send a close window command to passivate after some delay        
            time::sleep(Duration::from_secs(4)).await;
            context.send_viewport_cmd(egui::ViewportCommand::Close);
        });
    }))?;

    // This test does not assert, it exists to ensure that passivate starts (the context_accessor is invoked) and exits without hanging
    // The test does not pass in the case where a timeout occurs
    // The timeout is configured in .config/nextest.toml 
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {
}
