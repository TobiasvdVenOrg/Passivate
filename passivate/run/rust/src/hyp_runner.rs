use std::collections::BTreeSet;
use std::fs;
use std::sync::Arc;

use async_trait::async_trait;
use camino::Utf8PathBuf;
use cargo_nextest::cargo_cli::CargoOptions;
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
use nextest_runner::test_output::ChildExecutionOutput;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::hyp_name_strategy::HypNameStrategy;
use passivate_model_bridge::hyp_report::HypReport;
use passivate_model_bridge::hyp_session_bridge::{SendHypBridge, SendOutputBridge};
use passivate_model_bridge::hyp_session_event::{ConsoleOutput, ConsoleOutputKind};
use passivate_model_bridge::hyp_state::HypState;
use passivate_model_bridge::output_report::OutputReport;

use crate::hyp_run_error::HypRunError;
use crate::model::{RustBridge, RustHyp, RustOutput};
use crate::nextest_cargo_options::{self, cargo_build_scope_options};
use crate::nextest_error::NextestError;

#[mockall::automock]
#[async_trait]
pub trait RunHyps
{
    async fn run_hyps<TTx>(&mut self, instrument_coverage: bool, tx: &mut TTx, filter: Vec<String>) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>;

    async fn run_hyp<TTx>(&mut self, hyp_id: HypId, update_snapshots: bool, tx: &mut TTx) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>;
}

#[derive(Clone)]
pub struct HypRunner
{
    working_dir: Utf8PathBuf,
    target_dir: Utf8PathBuf,
    coverage_output_dir: Utf8PathBuf
}

impl HypRunner
{
    pub fn new(working_dir: Utf8PathBuf, target_dir: Utf8PathBuf, coverage_output_dir: Utf8PathBuf) -> Self
    {
        Self {
            working_dir,
            target_dir,
            coverage_output_dir
        }
    }

    async fn run_hyps_with_options<TTx>(
        &mut self,
        options: CargoOptions,
        instrument_coverage: bool,
        tx: &mut TTx,
        filter: Vec<String>
    ) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = dunce::canonicalize(&self.coverage_output_dir)?;

        unsafe {
            if instrument_coverage
            {
                std::env::set_var("RUSTFLAGS", "-C instrument-coverage");
                std::env::set_var("LLVM_PROFILE_FILE", coverage_output_dir.join("coverage-%p-%m.profraw"));
            }
        }

        let result = self.run_hyps_internal(options, tx, filter).await;

        unsafe {
            std::env::remove_var("RUSTFLAGS");
            std::env::remove_var("LLVM_PROFILE_FILE");
        }

        result
    }

    async fn run_hyps_internal<TTx>(
        &mut self,
        options: CargoOptions,
        tx: &mut TTx,
        filter: Vec<String>
    ) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        log::info!("Starting test run");

        let build_platforms = BuildPlatforms::new_with_no_target().map_err(NextestError::HostPlatformDetect)?;

        let output_context = OutputContext {
            verbose: false,
            color: Color::Auto
        };

        let manifest_path = self.working_dir.join("Cargo.toml");
        let graph_data = acquire_graph_data(
            Some(&manifest_path),
            Some(&self.target_dir),
            &options,
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
            self.working_dir.clone(),
            &parse_context,
            config_file,
            tool_config_files,
            &experimental
        )
        .map_err(NextestError::ConfigParse)?;

        let binary_list = options
            .compute_binary_list("test", &graph, Some(&manifest_path), output_context, build_platforms.clone())
            .map_err(NextestError::Expected)?;

        let path_mapper = PathMapper::noop();
        let rust_build_meta = binary_list.rust_build_meta.map_paths(&path_mapper);
        let platform_filter = None;
        let artifacts =
            RustTestArtifact::from_binary_list(&graph, Arc::new(binary_list), &rust_build_meta, &path_mapper, platform_filter)
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

        let test_filter = if filter.is_empty()
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
                    Filterset::parse(format!("test(={})", pattern), &parse_context, FiltersetKind::Test).map_err(|error| {
                        error
                            .errors
                            .into_iter()
                            .next()
                            .map_or_else(|| NextestError::UnknownFiltersetParse, NextestError::FiltersetParse)
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
            self.working_dir.clone(),
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
                    nextest_runner::reporter::events::ReporterEvent::Tick => todo!(),
                    nextest_runner::reporter::events::ReporterEvent::Test(test_event) => fun_name(tx, *test_event)
                };
            })
            .map_err(NextestError::TestRunnerExecute)?;

        log::info!("Completed test run");

        Ok(())
    }
}

fn fun_name<TTx>(tx: &mut TTx, test_event: TestEvent<'_>)
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

            let hyp_id = HypId::new(
                test_instance.binary_id.as_str(),
                test_instance.test_name.as_str().split("::").next().unwrap(),
                test_instance.test_name.as_str().split("::").skip(1).join("::")
            );

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

#[async_trait]
impl RunHyps for HypRunner
{
    async fn run_hyps<TTx>(&mut self, instrument_coverage: bool, tx: &mut TTx, filter: Vec<String>) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        let cargo_options = nextest_cargo_options::cargo_options()
            .target_dir(self.target_dir.clone())
            .call();

        self.run_hyps_with_options(cargo_options, instrument_coverage, tx, filter)
            .await
    }

    async fn run_hyp<TTx>(&mut self, hyp_id: HypId, update_snapshots: bool, tx: &mut TTx) -> Result<(), HypRunError>
    where
        TTx: SendHypBridge<RustBridge> + SendOutputBridge<RustBridge>
    {
        let instrument_coverage = false;
        let strategy = HypNameStrategy::QualifiedWithoutCrate {
            separator: "::".to_string()
        };

        let filter = vec![hyp_id.name(&strategy).to_string()];

        if update_snapshots
        {
            unsafe {
                std::env::set_var("UPDATE_SNAPSHOTS", "1");
            }
        }

        let cargo_scope = cargo_build_scope_options().all_features(true).call();

        let cargo_options = nextest_cargo_options::cargo_options()
            .build_scope(cargo_scope)
            .target_dir(self.target_dir.clone())
            .call();

        let result = self
            .run_hyps_with_options(cargo_options, instrument_coverage, tx, filter)
            .await;

        unsafe {
            std::env::set_var("UPDATE_SNAPSHOTS", "0");
        }

        result
    }
}
