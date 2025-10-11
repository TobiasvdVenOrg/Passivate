use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::sync::Arc;

use camino::Utf8PathBuf;
use guppy::graph::PackageGraph;
use nextest_filtering::{Filterset, FiltersetKind, ParseContext};
use nextest_runner::cargo_cli::{CargoOptions, acquire_graph_data};
use nextest_runner::cargo_config::{CargoConfigs, EnvironmentMap};
use nextest_runner::config::core::{NextestConfig, get_num_cpus};
use nextest_runner::config::elements::MaxFail;
use nextest_runner::double_spawn::DoubleSpawnInfo;
use nextest_runner::input::InputHandlerKind;
use nextest_runner::list::{RustTestArtifact, TestExecuteContext, TestList};
use nextest_runner::platform::BuildPlatforms;
use nextest_runner::reporter::FinalStatusLevel;
use nextest_runner::reuse_build::PathMapper;
use nextest_runner::runner::TestRunnerBuilder;
use nextest_runner::signal::SignalHandlerKind;
use nextest_runner::target_runner::TargetRunner;
use nextest_runner::test_filter::{FilterBound, RunIgnored, TestFilterBuilder, TestFilterPatterns};
use nextest_runner::test_output::ChildExecutionOutput;
use passivate_delegation::{Cancellation, Tx};
use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};

use super::TestRunError;
use crate::passivate_nextest::cargo_options;
use crate::test_run_model::{SingleTest, SingleTestStatus, TestRun, TestRunEvent};

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

    pub fn run_hyps(
        &mut self,
        instrument_coverage: bool,
        cancellation: Cancellation,
        sender: &mut Tx<TestRun>,
        filter: Vec<String>,
        snapshots_path: Option<Utf8PathBuf>
    ) -> Result<(), TestRunError>
    {
        if self.test_run.update(TestRunEvent::Start)
        {
            sender.send(self.test_run.clone());
        }

        let cargo_options = cargo_options().target_dir(self.target_dir.clone()).call();

        self.run_hyps_with_options(cargo_options, instrument_coverage, cancellation, sender, filter, snapshots_path)
    }

    fn run_hyps_with_options(
        &mut self,
        options: CargoOptions,
        instrument_coverage: bool,
        cancellation: Cancellation,
        sender: &mut Tx<TestRun>,
        filter: Vec<String>,
        snapshots_path: Option<Utf8PathBuf>
    ) -> Result<(), TestRunError>
    {
        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = dunce::canonicalize(&self.coverage_output_dir)?;

        unsafe {
            if let Some(snapshots_path) = snapshots_path
            {
                std::env::set_var("PASSIVATE_SNAPSHOT_DIR", snapshots_path);
            }

            if instrument_coverage
            {
                std::env::set_var("RUSTFLAGS", "-C instrument-coverage");
                std::env::set_var("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"));
            }
        }

        let result = self.run_hyps_internal(options, cancellation, sender, filter);

        unsafe {
            std::env::remove_var("PASSIVATE_SNAPSHOT_DIR");
            std::env::remove_var("RUSTFLAGS");
            std::env::remove_var("LLVM_PROFILE_FILE");
        }

        result
    }

    fn run_hyps_internal(&mut self, options: CargoOptions, cancellation: Cancellation, sender: &mut Tx<TestRun>, filter: Vec<String>) -> Result<(), TestRunError>
    {
        let build_platforms = BuildPlatforms::new_with_no_target().map_err(|error| {
            eprintln!("3 {:?}", error);
            TestRunError::Temp
        })?;

        let manifest_path = self.working_dir.join("Cargo.toml");
        let graph_data = acquire_graph_data(Some(&manifest_path), Some(&self.target_dir), &options, &build_platforms).map_err(|error| TestRunError::Temp)?;
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

        let binary_list = options
            .compute_binary_list(&graph, Some(&manifest_path), build_platforms.clone())
            .map_err(|error| TestRunError::Temp)?;

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

        let test_filter_builder = if filter.is_empty()
        {
            TestFilterBuilder::default_set(RunIgnored::Default)
        }
        else
        {
            let patterns = TestFilterPatterns::new(Vec::new());

            let mut filter_sets = vec![];

            for pattern in filter
            {
                let filterset = Filterset::parse(format!("test(={})", pattern), &parse_context, FiltersetKind::Test).map_err(|error| TestRunError::Temp)?;
                filter_sets.push(filterset);
            }

            let partitioner_builder = None;

            TestFilterBuilder::new(RunIgnored::Default, partitioner_builder, patterns, filter_sets).map_err(|error| TestRunError::Temp)?
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

        let mut runner_builder = TestRunnerBuilder::default();
        runner_builder.set_max_fail(MaxFail::from_fail_fast(false));

        let runner = runner_builder
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
                let event = match test_event.kind
                {
                    nextest_runner::reporter::events::TestEventKind::RunStarted {
                        test_list: _,
                        run_id: _,
                        profile_name: _,
                        cli_args: _,
                        stress_condition: _
                    } => None,
                    nextest_runner::reporter::events::TestEventKind::TestStarted {
                        stress_index: _,
                        test_instance,
                        current_stats: _,
                        running: _
                    } => None,
                    nextest_runner::reporter::events::TestEventKind::TestFinished {
                        stress_index: _,
                        test_instance,
                        success_output: _,
                        failure_output: _,
                        junit_store_success_output: _,
                        junit_store_failure_output: _,
                        run_statuses,
                        current_stats: _,
                        running: _
                    } =>
                    {
                        let test_output: Vec<String> = run_statuses
                            .iter()
                            .flat_map(|status| {
                                if let ChildExecutionOutput::Output { result: _, output, errors: _ } = &status.output
                                {
                                    match output
                                    {
                                        nextest_runner::test_output::ChildOutput::Split(child_split_output) =>
                                        {
                                            if let Some(stderr) = &child_split_output.stderr
                                            {
                                                stderr.lines().map(|l| String::from_utf8(l.to_vec()).unwrap()).collect()
                                            }
                                            else
                                            {
                                                Vec::new()
                                            }
                                        }
                                        nextest_runner::test_output::ChildOutput::Combined { output } => Vec::new()
                                    }
                                }
                                else
                                {
                                    Vec::new()
                                }
                            })
                            .collect();

                        let status = if let FinalStatusLevel::Pass = run_statuses.describe().final_status_level()
                        {
                            SingleTestStatus::Passed
                        }
                        else
                        {
                            SingleTestStatus::Failed
                        };

                        let hyp_id = HypId::new(test_instance.suite_info.binary_name.clone(), test_instance.name).expect("todo: error handling");
                        Some(TestRunEvent::TestFinished(SingleTest::new(hyp_id, status, test_output)))
                    }
                    nextest_runner::reporter::events::TestEventKind::RunFinished {
                        run_id: _,
                        start_time: _,
                        elapsed: _,
                        run_stats: _
                    } => Some(TestRunEvent::TestsCompleted),
                    _ => None
                };

                if let Some(event) = event
                    && self.test_run.update(event.clone())
                {
                    sender.send(self.test_run.clone());
                }
            })
            .map_err(|error| TestRunError::Temp)?;

        Ok(())
    }

    pub fn run_hyp(
        &mut self,
        hyp_id: &HypId,
        update_snapshots: bool,
        cancellation: Cancellation,
        sender: &mut Tx<TestRun>,
        snapshots_path: Option<Utf8PathBuf>
    ) -> Result<(), TestRunError>
    {
        let instrument_coverage = false;
        let strategy = HypNameStrategy::QualifiedWithoutCrate { separator: "::".to_string() };
        let filter = vec![hyp_id.get_name(&strategy).to_string()];

        if self.test_run.update(TestRunEvent::StartSingle {
            hyp: hyp_id.clone(),
            clear_tests: true
        })
        {
            sender.send(self.test_run.clone());
        }

        if update_snapshots
        {
            unsafe {
                std::env::set_var("UPDATE_SNAPSHOTS", "1");
            }
        }

        let cargo_options = cargo_options().all_features(true).target_dir(self.target_dir.clone()).call();

        let result = self.run_hyps_with_options(cargo_options, instrument_coverage, cancellation, sender, filter, snapshots_path);

        unsafe {
            std::env::set_var("UPDATE_SNAPSHOTS", "0");
        }

        result
    }
}
