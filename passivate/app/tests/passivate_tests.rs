use libtest_mimic::Failed;
use passivate_hyp_names::test_name;
use passivate_run_rust::hyp_run_handler;
use passivate_testing::test_data_setup::TestDataSetup;

fn main()
{
    use libtest_mimic::{Arguments, Trial};

    let mut args = Arguments::from_args();

    let tests = vec![Trial::test("start_and_exit_passivate", start_and_exit_passivate)];

    args.test_threads = Some(1);

    // We use libtest-mimic because it allows us to run our tests on the main thread
    // This is a requirement for these tests, which actually start passivate
    libtest_mimic::run(&args, tests).exit();
}

pub fn start_and_exit_passivate() -> Result<(), Failed>
{
    use std::time::Duration;

    use passivate::start::run_app_and_get_context;
    use passivate_core::{compose::compose, passivate_args::PassivateArgs};
    use tokio::time;

    let setup = TestDataSetup::builder(test_name!(), "simple_project").build();

    let args = PassivateArgs::builder()
        .root_directory(setup.workspace_path())
        .target_directory(setup.output_path())
        .build();

    let runtime = hyp_run_handler::build_tokio_runtime();

    let passivate = compose(args, &runtime)?;

    run_app_and_get_context(passivate,
        Box::new(|context: egui::Context| {
            runtime.spawn(async move {
                // Asynchronously send a close window command to passivate after some delay
                time::sleep(Duration::from_secs(4)).await;

                context.send_viewport_cmd(egui::ViewportCommand::Close);
            });
        })
    )
    .map_err(|error|
    {
        eprintln!("{:?}", error);
        error
    })?;

    // This test does not assert, it exists to ensure that passivate starts (the context_accessor is invoked) and exits without hanging
    // The test does not pass in the case where a timeout occurs
    // The timeout is configured in .config/nextest.toml
    Ok(())
}
