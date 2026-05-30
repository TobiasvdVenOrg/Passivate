use std::collections::BTreeSet;
use std::fs;
use std::sync::Arc;

use camino::Utf8PathBuf;
use cargo_nextest::dispatch::helpers::acquire_graph_data;
use cargo_nextest::output::{Color, OutputContext};
use guppy::graph::PackageGraph;
use itertools::Itertools;
use nextest_filtering::{Filterset, FiltersetKind, ParseContext};
use nextest_runner::cargo_config::{CargoConfigs, EnvironmentMap};
use nextest_runner::config::core::{NextestConfig, get_num_cpus};
use nextest_runner::config::elements::MaxFail;
use nextest_runner::double_spawn::DoubleSpawnInfo;
use nextest_runner::input::InputHandlerKind;
use nextest_runner::list::{RustTestArtifact, TestExecuteContext, TestList};
use nextest_runner::platform::BuildPlatforms;
use nextest_runner::reporter::FinalStatusLevel;
use nextest_runner::reporter::events::{ChildExecutionOutputDescription, TestEvent};
use nextest_runner::reuse_build::PathMapper;
use nextest_runner::run_mode::NextestRunMode;
use nextest_runner::runner::TestRunnerBuilder;
use nextest_runner::signal::SignalHandlerKind;
use nextest_runner::target_runner::TargetRunner;
use nextest_runner::test_filter::{FilterBound, RunIgnored, TestFilter, TestFilterPatterns};
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::hyp_name_strategy::HypNameStrategy;
use passivate_model_bridge::hyp_report::HypReport;
use passivate_model_bridge::hyp_session_bridge::{SendHypBridge, SendOutputBridge};
use passivate_model_bridge::hyp_session_event::{ConsoleOutput, ConsoleOutputKind};
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_bridge::output_report::OutputReport;

use crate::hyp_run_error::HypRunError;
use crate::model::{RustBridge, RustHyp, RustOutput};
use crate::nextest_cargo_options;
use crate::nextest_error::NextestError;

#[derive(bon::Builder)]
pub struct RunHypsOptions
{
    pub manifest_dir: Utf8PathBuf,
    pub target_dir: Utf8PathBuf,
    pub coverage_dir: Option<Utf8PathBuf>,
    pub update_snapshots: bool
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait RunHyps
{
    async fn run_hyps<TTx>(&mut self, options: &RunHypsOptions, tx: &mut TTx) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>;

    async fn run_hyp<TTx>(&mut self, hyp_id: HypId, options: &RunHypsOptions, tx: &mut TTx) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>;
}

#[derive(Clone)]
pub struct HypRunner;

#[async_trait::async_trait]
impl RunHyps for HypRunner
{
    async fn run_hyps<TTx>(&mut self, options: &RunHypsOptions, tx: &mut TTx) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        let filter = vec![];
        self.run_hyps_with_options(options, filter, tx).await
    }

    async fn run_hyp<TTx>(&mut self, hyp_id: HypId, options: &RunHypsOptions, tx: &mut TTx) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        let strategy = HypNameStrategy::QualifiedWithoutCrate {
            separator: "::".to_string()
        };

        let filter = vec![hyp_id.name(&strategy).to_string()];

        let result = self.run_hyps_with_options(options, filter, tx).await;

        result
    }
}

impl HypRunner
{
    async fn run_hyps_with_options<TTx>(
        &mut self,
        options: &RunHypsOptions,
        filter: Vec<String>,
        tx: &mut TTx
    ) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        if let Some(coverage_dir) = &options.coverage_dir
        {
            fs::create_dir_all(coverage_dir)?;
            let coverage_output_dir = dunce::canonicalize(coverage_dir)?;

            unsafe {
                std::env::set_var("RUSTFLAGS", "-C instrument-coverage");
                std::env::set_var("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"));
            }
        }

        if options.update_snapshots
        {
            unsafe {
                std::env::set_var("UPDATE_SNAPSHOTS", "1");
            }
        }

        let result = self.run_hyps_internal(options, filter, tx).await;

        unsafe {
            std::env::remove_var("RUSTFLAGS");
            std::env::remove_var("LLVM_PROFILE_FILE");
            std::env::set_var("UPDATE_SNAPSHOTS", "0");
        }

        result
    }

    async fn run_hyps_internal<TTx>(
        &mut self,
        options: &RunHypsOptions,
        filter: Vec<String>,
        tx: &mut TTx
    ) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        log::info!("Starting test run");

        std::thread::scope(|scope| {
            scope
                .spawn(move || {
                    let cargo_options = nextest_cargo_options::cargo_options()
                        .target_dir(options.target_dir.clone())
                        .call();

                    let build_platforms = BuildPlatforms::new_with_no_target().map_err(NextestError::HostPlatformDetect)?;

                    let output_context = OutputContext {
                        verbose: false,
                        color: Color::Auto
                    };

                    let manifest_path = options.manifest_dir.join("Cargo.toml");

                    log::info!("querying metadata for: {manifest_path}");

                    let graph_data = acquire_graph_data(
                        Some(&manifest_path),
                        Some(&options.target_dir),
                        &cargo_options,
                        &build_platforms,
                        output_context
                    )
                    .map_err(NextestError::Expected)?;

                    let graph = PackageGraph::from_json(graph_data)?;

                    log::info!("Completed 'metadata'");

                    let parse_context = ParseContext::new(&graph);
                    let config_file = None;
                    let tool_config_files = Vec::new();
                    let experimental = BTreeSet::new();
                    let nextest_config = NextestConfig::from_sources(
                        options.manifest_dir.as_path(),
                        &parse_context,
                        config_file,
                        tool_config_files,
                        &experimental
                    )
                    .map_err(NextestError::ConfigParse)?;

                    let binary_list = cargo_options
                        .compute_binary_list("test", &graph, Some(&manifest_path), output_context, build_platforms.clone())
                        .map_err(NextestError::Expected)?;

                    let path_mapper = PathMapper::noop();
                    let rust_build_meta = binary_list.rust_build_meta.map_paths(&path_mapper);
                    let platform_filter = None;
                    let artifacts = RustTestArtifact::from_binary_list(
                        &graph,
                        Arc::new(binary_list),
                        &rust_build_meta,
                        &path_mapper,
                        platform_filter
                    )
                    .map_err(NextestError::FromMessages)?;

                    let double_spawn = DoubleSpawnInfo::disabled();
                    let target_runner = TargetRunner::empty();

                    let profile = nextest_config
                        .profile(NextestConfig::DEFAULT_PROFILE)
                        .map_err(NextestError::ProfileNotFound)?;

                    let profile = profile.apply_build_platforms(&build_platforms);

                    let context = TestExecuteContext {
                        profile_name: NextestConfig::DEFAULT_PROFILE,
                        double_spawn: &double_spawn,
                        target_runner: &target_runner
                    };

                    let test_filter =
                        if filter.is_empty()
                        {
                            TestFilter::default_set(NextestRunMode::Test, RunIgnored::Default)
                        }
                        else
                        {
                            let patterns = TestFilterPatterns::new(Vec::new());

                            let mut filter_sets = vec![];

                            for pattern in filter
                            {
                                let filterset =
                                    Filterset::parse(format!("test(={})", pattern), &parse_context, FiltersetKind::Test)
                                        .map_err(|error| {
                                            error.errors.into_iter().next().map_or_else(
                                                || NextestError::UnknownFiltersetParse,
                                                NextestError::FiltersetParse
                                            )
                                        })?;
                                filter_sets.push(filterset);
                            }

                            TestFilter::new(NextestRunMode::Test, RunIgnored::Default, patterns, filter_sets)
                                .map_err(NextestError::TestFilterBuild)?
                        };

                    let cli_configs: Vec<String> = Vec::new();
                    let cargo_configs = CargoConfigs::new(cli_configs.into_iter()).map_err(NextestError::CargoConfig)?;

                    let env = EnvironmentMap::new(&cargo_configs);

                    let partitioner_builder = None;
                    let test_list = TestList::new(
                        &context,
                        artifacts.into_iter(),
                        rust_build_meta,
                        &test_filter,
                        partitioner_builder,
                        options.manifest_dir.clone(),
                        env,
                        &profile,
                        FilterBound::DefaultSet,
                        get_num_cpus()
                    )
                    .map_err(NextestError::CreateTestList)?;

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
                        .map_err(NextestError::TestRunnerBuild)?;

                    runner
                        .execute(|test_event| {
                            match test_event
                            {
                                nextest_runner::reporter::events::ReporterEvent::Tick =>
                                {}
                                nextest_runner::reporter::events::ReporterEvent::Test(test_event) =>
                                {
                                    process_nextest_event(tx, *test_event)
                                }
                            };
                        })
                        .map_err(NextestError::TestRunnerExecute)?;

                    log::info!("Completed test run");

                    Ok(())
                })
                .join()
        })
        .unwrap()
    }
}

fn process_nextest_event<TTx>(tx: &mut TTx, test_event: TestEvent<'_>)
where
    TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
{
    match test_event.kind
    {
        nextest_runner::reporter::events::TestEventKind::RunStarted {
            test_list: _,
            run_id: _,
            profile_name: _,
            cli_args: _,
            stress_condition: _
        } =>
        {}
        nextest_runner::reporter::events::TestEventKind::TestStarted {
            stress_index: _,
            test_instance: _,
            current_stats: _,
            running: _,
            command_line: _
        } =>
        {}
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
                    match &status.output
                    {
                        ChildExecutionOutputDescription::Output { result, output, errors } =>
                        {
                            match output
                            {
                                nextest_runner::reporter::events::ChildOutputDescription::Split { stdout, stderr } =>
                                {
                                    if let Some(stderr) = &stderr
                                    {
                                        stderr.lines().map(|l| String::from_utf8(l.to_vec()).unwrap()).collect()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    }
                                }
                                nextest_runner::reporter::events::ChildOutputDescription::Combined { output } => todo!()
                            }
                        }
                        ChildExecutionOutputDescription::StartError(child_start_error_description) => todo!()
                    }
                })
                .collect();

            let state = if let FinalStatusLevel::Pass = run_statuses.describe().final_status_level()
            {
                HypState::Passed
            }
            else
            {
                HypState::Failed
            };

            let mut binary_id = test_instance.binary_id.as_str().split("::");

            let package_id = binary_id.next().unwrap();
            let crate_id = binary_id.next().unwrap_or(package_id);

            let hyp_id = HypId::new(package_id, crate_id, test_instance.test_name.as_str());

            let hyp_info = RustHyp::new_single(hyp_id.clone());
            let hyp_report = HypReport::new_fixed(hyp_info, state);

            tx.send_hyp(hyp_report);

            for line in test_output
            {
                let output_report = OutputReport::new(
                    hyp_id.clone(),
                    RustOutput::Console(ConsoleOutput {
                        content: line,
                        kind: ConsoleOutputKind::StdErr
                    })
                );

                tx.send_output(output_report);
            }
        }
        nextest_runner::reporter::events::TestEventKind::RunFinished {
            run_id: _,
            start_time: _,
            elapsed: _,
            run_stats: _,
            outstanding_not_seen: _
        } =>
        {}
        _ =>
        {}
    };
}
