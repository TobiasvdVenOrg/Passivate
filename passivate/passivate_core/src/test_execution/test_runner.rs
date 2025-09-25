use std::collections::BTreeSet;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::io::Cursor;
use std::sync::Arc;

use camino::Utf8PathBuf;
use duct::cmd;
use guppy::graph::PackageGraph;
use nextest_filtering::ParseContext;
use nextest_runner::cargo_config::{CargoConfigs, EnvironmentMap};
use nextest_runner::config::core::{NextestConfig, get_num_cpus};
use nextest_runner::double_spawn::DoubleSpawnInfo;
use nextest_runner::input::InputHandlerKind;
use nextest_runner::list::{BinaryList, RustTestArtifact, TestExecuteContext, TestList};
use nextest_runner::platform::BuildPlatforms;
use nextest_runner::reuse_build::PathMapper;
use nextest_runner::signal::SignalHandlerKind;
use nextest_runner::target_runner::TargetRunner;
use nextest_runner::test_filter::{FilterBound, RunIgnored, TestFilterBuilder, TestFilterPatterns};
use nextest_runner::reporter::FinalStatusLevel;
use passivate_delegation::{Cancellation, Tx};

use super::TestRunError;
use crate::test_run_model::{TestId, TestRun, TestRunEvent, SingleTest, SingleTestStatus};

#[faux::create]
#[derive(Clone)]
pub struct TestRunner
{
    target: OsString,
    working_dir: Utf8PathBuf,
    target_dir: Utf8PathBuf,
    coverage_output_dir: Utf8PathBuf,
    test_run: TestRun
}

#[faux::methods]
impl TestRunner
{
    pub fn new(target: OsString, working_dir: Utf8PathBuf, target_dir: Utf8PathBuf, coverage_output_dir: Utf8PathBuf, test_run: TestRun) -> Self
    {
        Self {
            target,
            working_dir,
            target_dir,
            coverage_output_dir,
            test_run
        }
    }

    // Unable to test effectively due to non-deterministic order of cargo test output (order of tests changes)
    // During manual testing stdout and stderr output appeared to be interleaved in the correct order
    pub fn run_tests(&mut self, instrument_coverage: bool, cancellation: Cancellation, sender: &mut Tx<TestRun>, filter: Vec<String>) -> Result<(), TestRunError>
    {
        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = dunce::canonicalize(&self.coverage_output_dir)?;

        let graph_data = self.acquire_graph_data()?;
        let graph = PackageGraph::from_json(graph_data).map_err(|error| {
            eprintln!("1 {:?}", error);
            TestRunError::Temp
        })?;

        let parse_context = ParseContext::new(&graph);
        let config_file = None;
        let tool_config_files = Vec::new();
        let experimental = BTreeSet::new();
        let nextest_config = NextestConfig::from_sources(self.working_dir.clone(), &parse_context, config_file, tool_config_files, &experimental).map_err(|error| {
            eprintln!("2 {:?}", error);
            TestRunError::Temp
        })?;

        let build_platforms = BuildPlatforms::new_with_no_target().map_err(|error| {
            eprintln!("3 {:?}", error);
            TestRunError::Temp
        })?;

        let binary_list = self.compute_binary_list(&graph, &build_platforms)?;
        let path_mapper = PathMapper::noop();
        let rust_build_meta = binary_list.rust_build_meta.map_paths(&path_mapper);
        let platform_filter = None;
        let artifacts = RustTestArtifact::from_binary_list(&graph, Arc::new(binary_list), &rust_build_meta, &path_mapper, platform_filter).map_err(|error| {
            eprintln!("4 {:?}", error);
            TestRunError::Temp
        })?;
        let double_spawn = DoubleSpawnInfo::disabled();
        let target_runner = TargetRunner::empty();

        let profile = nextest_config.profile(NextestConfig::DEFAULT_PROFILE).map_err(|error| {
            eprintln!("5 {:?}", error);
            TestRunError::Temp
        })?;
        let profile = profile.apply_build_platforms(&build_platforms);

        let context = TestExecuteContext {
            profile_name: NextestConfig::DEFAULT_PROFILE,
            double_spawn: &double_spawn,
            target_runner: &target_runner
        };

        let partitioner_builder = None;
        let test_filter_expressions = vec![];

        let test_filter_builder = if filter.is_empty()
        {
            TestFilterBuilder::default_set(RunIgnored::Default)
        }
        else
        {
            let c = filter.clone();
            let mut patterns = TestFilterPatterns::new(filter);
            
            for f in c
            {
                println!("f: {:?}", f);

                patterns.add_exact_pattern(f);
            }

            TestFilterBuilder::new(RunIgnored::Default, partitioner_builder, patterns,  test_filter_expressions).map_err(|error| TestRunError::Temp)?
        };

        let cli_configs: Vec<String> = Vec::new();
        let cargo_configs = CargoConfigs::new(cli_configs.into_iter()).map_err(|error| {
            eprintln!("6 {:?}", error);
            TestRunError::Temp
        })?;

        let env = EnvironmentMap::new(&cargo_configs);

        let test_list = TestList::new(
            &context,
            artifacts.into_iter(),
            rust_build_meta,
            &test_filter_builder,
            self.working_dir.clone(),
            env,
            &profile,
            FilterBound::DefaultSet,
            get_num_cpus()
        )
        .map_err(|error| {
            eprintln!("7 {:?}", error);
            TestRunError::Temp
        })?;

        let runner = nextest_runner::runner::TestRunnerBuilder::default()
            .build(
                &test_list,
                &profile,
                vec![], // we aren't testing CLI args at the moment
                SignalHandlerKind::Noop,
                InputHandlerKind::Noop,
                DoubleSpawnInfo::disabled(),
                TargetRunner::empty()
            )
            .map_err(|error| {
                eprintln!("8 {:?}", error);
                TestRunError::Temp
            })?;

        runner
            .execute(|test_event| {
                let dbg = format!("{:?}", test_event.kind);
                println!("test_event: {:?}", dbg.split_once("{").unwrap().0);

                let event = match test_event.kind
                {
                    nextest_runner::reporter::events::TestEventKind::RunStarted {
                        test_list,
                        run_id,
                        profile_name,
                        cli_args,
                        stress_condition
                    } => Some(TestRunEvent::Start),
                    nextest_runner::reporter::events::TestEventKind::StressSubRunStarted { progress } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::SetupScriptStarted {
                        stress_index,
                        index,
                        total,
                        script_id,
                        program,
                        args,
                        no_capture
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::SetupScriptSlow {
                        stress_index,
                        script_id,
                        program,
                        args,
                        elapsed,
                        will_terminate
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::SetupScriptFinished {
                        stress_index,
                        index,
                        total,
                        script_id,
                        program,
                        args,
                        junit_store_success_output,
                        junit_store_failure_output,
                        no_capture,
                        run_status
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::TestStarted {
                        stress_index,
                        test_instance,
                        current_stats,
                        running
                    } => Some(TestRunEvent::StartSingle { test: TestId::new(test_instance.name), clear_tests: false }),
                    nextest_runner::reporter::events::TestEventKind::TestSlow {
                        stress_index,
                        test_instance,
                        retry_data,
                        elapsed,
                        will_terminate
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::TestAttemptFailedWillRetry {
                        stress_index,
                        test_instance,
                        run_status,
                        delay_before_next_attempt,
                        failure_output
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::TestRetryStarted {
                        stress_index,
                        test_instance,
                        retry_data
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::TestFinished {
                        stress_index,
                        test_instance,
                        success_output,
                        failure_output,
                        junit_store_success_output,
                        junit_store_failure_output,
                        run_statuses,
                        current_stats,
                        running
                    } => 
                    {
                        let (status, output) = if let FinalStatusLevel::Pass = run_statuses.describe().final_status_level()
                        {
                            (SingleTestStatus::Passed, success_output)
                        }
                        else
                        {
                            (SingleTestStatus::Failed, failure_output)
                        };

                        Some(TestRunEvent::TestFinished(SingleTest::new(test_instance.name.to_string(), status, vec!["MISSING OUTPUT".to_string()])))
                    },
                    nextest_runner::reporter::events::TestEventKind::TestSkipped {
                        stress_index,
                        test_instance,
                        reason
                    } => None,
                    nextest_runner::reporter::events::TestEventKind::InfoStarted { total, run_stats } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::InfoResponse { index, total, response } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::InfoFinished { missing } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::InputEnter { current_stats, running } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::RunBeginCancel {
                        setup_scripts_running,
                        current_stats,
                        running
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::RunBeginKill {
                        setup_scripts_running,
                        current_stats,
                        running
                    } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::RunPaused { setup_scripts_running, running } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::RunContinued { setup_scripts_running, running } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::StressSubRunFinished { progress, sub_elapsed, sub_stats } => todo!(),
                    nextest_runner::reporter::events::TestEventKind::RunFinished {
                        run_id,
                        start_time,
                        elapsed,
                        run_stats
                    } => Some(TestRunEvent::TestsCompleted)
                };

                if let Some(event) = event
                    && self.test_run.update(event.clone())
                    {
                        println!("send: {:?}", &event);
                        sender.send(self.test_run.clone());
                    }
            })
            .map_err(|error| TestRunError::Temp)?;

        Ok(())
    }

    pub fn run_test(&mut self, test_name: &TestId, update_snapshots: bool, cancellation: Cancellation, sender: &mut Tx<TestRun>) -> Result<(), TestRunError>
    {
        let instrument_coverage = false;
        let filter = vec![ test_name.get_name() ];
        
        if self.test_run.update(TestRunEvent::StartSingle { test: test_name.clone(), clear_tests: true })
        {
            sender.send(self.test_run.clone());
        }

        self.run_tests(instrument_coverage, cancellation, sender, filter)
    }

    fn acquire_graph_data(&self) -> Result<String, TestRunError>
    {
        //.add_args(["--filter-platform", &cargo_target_arg_str])

        let mut args: Vec<OsString> = vec![];

        args.push(OsString::from("metadata"));
        args.push(OsString::from("--format-version=1"));
        args.push(OsString::from("--all-features"));

        // cargo metadata doesn't support "--target-dir" but setting the environment
        // variable works.
        let command = cmd("cargo", args).dir(self.working_dir.clone()).env("CARGO_TARGET_DIR", self.target_dir.as_os_str());

        let output = command.stdout_capture().unchecked().run()?;

        if !output.status.success()
        {
            return Err(TestRunError::Io(std::io::Error::new(std::io::ErrorKind::AddrInUse, "")));
        }

        let json = String::from_utf8(output.stdout).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;

        Ok(json)
    }

    fn compute_binary_list(&self, graph: &PackageGraph, build_platforms: &BuildPlatforms) -> Result<BinaryList, TestRunError>
    {
        let mut args: Vec<OsString> = vec![];

        args.push(OsString::from("test"));
        args.push(OsString::from("--no-run"));
        args.push(OsString::from("--message-format"));
        args.push(OsString::from("json-render-diagnostics"));

        let command = cmd("cargo", args).dir(&self.working_dir);

        let output = command.stdout_capture().unchecked().run()?;

        if !output.status.success()
        {
            return Err(TestRunError::Io(std::io::Error::new(std::io::ErrorKind::AddrInUse, "")));
        }

        let test_binaries = BinaryList::from_messages(Cursor::new(output.stdout), graph, build_platforms.clone()).map_err(|error| {
            eprintln!("{:?}", error);
            TestRunError::Io(std::io::Error::new(std::io::ErrorKind::AddrInUse, ""))
        })?;

        Ok(test_binaries)
    }
}
