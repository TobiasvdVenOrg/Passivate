use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io::Cursor;
use std::sync::Arc;

use camino::Utf8PathBuf;
use duct::cmd;
use guppy::graph::PackageGraph;
use nextest_filtering::ParseContext;
use nextest_runner::cargo_config::{CargoConfigs, EnvironmentMap};
use nextest_runner::config::core::{get_num_cpus, NextestConfig};
use nextest_runner::double_spawn::DoubleSpawnInfo;
use nextest_runner::input::InputHandlerKind;
use nextest_runner::list::{BinaryList, RustTestArtifact, TestExecuteContext, TestList};
use nextest_runner::platform::BuildPlatforms;
use nextest_runner::reuse_build::PathMapper;
use nextest_runner::signal::SignalHandlerKind;
use nextest_runner::target_runner::TargetRunner;
use nextest_runner::test_filter::{FilterBound, RunIgnored, TestFilterBuilder};
use passivate_delegation::Cancellation;

use super::TestRunError;

#[faux::create] 
pub struct TestRunner
{
    target: OsString,
    working_dir: Utf8PathBuf,
    target_dir: Utf8PathBuf,
    coverage_output_dir: Utf8PathBuf
}

#[faux::methods]
impl TestRunner
{
    pub fn new(
        target: OsString,
        working_dir: Utf8PathBuf,
        target_dir: Utf8PathBuf,
        coverage_output_dir: Utf8PathBuf) -> Self
    {
        Self {
            target,
            working_dir,
            target_dir,
            coverage_output_dir
        }
    }

    // Unable to test effectively due to non-deterministic order of cargo test output (order of tests changes)
    // During manual testing stdout and stderr output appeared to be interleaved in the correct order
    pub fn run_tests(&self, instrument_coverage: bool, cancellation: Cancellation) -> Result<(), TestRunError>
    {
        eprintln!("RUN TESTS");
        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = dunce::canonicalize(&self.coverage_output_dir)?;
        eprintln!("RUN TESTS 2");
        let graph_data = self.acquire_graph_data()?;
        let graph = PackageGraph::from_json(graph_data).map_err(|error| {
            eprintln!("1 {:?}", error);
            TestRunError::Temp
        })?;

        fs::write("D:\\graph.txt", format!("{:?}", graph)).expect("graph.txt");

        let parse_context = ParseContext::new(&graph);
        let config_file = None;
        let tool_config_files = Vec::new();
        let experimental = BTreeSet::new();
        let nextest_config = NextestConfig::from_sources(self.working_dir.clone(), &parse_context, config_file, tool_config_files, &experimental).map_err(|error|  {
            eprintln!("2 {:?}", error);
            TestRunError::Temp
        })?;
        let build_platforms = BuildPlatforms::new_with_no_target().map_err(|error|  {
            eprintln!("3 {:?}", error);
            TestRunError::Temp
        })?;
        let binary_list = self.compute_binary_list(&graph, &build_platforms)?;
        eprintln!("RUN TESTS3");
        let path_mapper = PathMapper::noop();
        let rust_build_meta = binary_list.rust_build_meta.map_paths(&path_mapper);
        let platform_filter = None;
        let artifacts = RustTestArtifact::from_binary_list(&graph, Arc::new(binary_list), &rust_build_meta, &path_mapper, platform_filter).map_err(|error|  {
            eprintln!("4 {:?}", error);
            TestRunError::Temp
        })?;
        let double_spawn = DoubleSpawnInfo::disabled();
        let target_runner = TargetRunner::empty();
        
        let profile = nextest_config.profile(NextestConfig::DEFAULT_PROFILE).map_err(|error|  {
            eprintln!("5 {:?}", error);
            TestRunError::Temp
        })?;
        let profile = profile.apply_build_platforms(&build_platforms);

        let context = TestExecuteContext {
            profile_name: NextestConfig::DEFAULT_PROFILE,
            double_spawn: &double_spawn,
            target_runner: &target_runner
        };

        let test_filter_builder = TestFilterBuilder::default_set(RunIgnored::Default);

        let cli_configs: Vec<String> = Vec::new();
        let cargo_configs = CargoConfigs::new(cli_configs.into_iter()).map_err(|error|  {
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
        ).map_err(|error|  {
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
            .map_err(|error|  {
            eprintln!("8 {:?}", error);
            TestRunError::Temp
        })?;

        eprintln!("LIST: {:?}", test_list);
        runner.execute(|test_event| {
            eprintln!("{:?}", test_event);
        }).map_err(|error| TestRunError::Temp)?;

        Ok(())
    }

    pub fn run_test(&self, test_name: &str, update_snapshots: bool, cancellation: Cancellation) -> Result<(), TestRunError>
    {
        fs::create_dir_all(&self.coverage_output_dir)?;
        let coverage_output_dir = dunce::canonicalize(&self.coverage_output_dir)?;
        
        let graph_data = self.acquire_graph_data()?;
        let graph = PackageGraph::from_json(graph_data).map_err(|error| TestRunError::Temp)?;

        let parse_context = ParseContext::new(&graph);
        let config_file = None;
        let tool_config_files = Vec::new();
        let experimental = BTreeSet::new();
        let nextest_config = NextestConfig::from_sources(self.working_dir.clone(), &parse_context, config_file, tool_config_files, &experimental).map_err(|error| TestRunError::Temp)?;
        let build_platforms = BuildPlatforms::new_with_no_target().map_err(|error| TestRunError::Temp)?;
        let binary_list = self.compute_binary_list(&graph, &build_platforms)?;
        let path_mapper = PathMapper::noop();
        let rust_build_meta = binary_list.rust_build_meta.map_paths(&path_mapper);
        let platform_filter = None;
        let artifacts = RustTestArtifact::from_binary_list(&graph, Arc::new(binary_list), &rust_build_meta, &path_mapper, platform_filter).map_err(|error| TestRunError::Temp)?;
        let double_spawn = DoubleSpawnInfo::disabled();
        let target_runner = TargetRunner::empty();
        
        let profile = nextest_config.profile(NextestConfig::DEFAULT_PROFILE).map_err(|error| TestRunError::Temp)?;
        let profile = profile.apply_build_platforms(&build_platforms);

        let context = TestExecuteContext {
            profile_name: NextestConfig::DEFAULT_PROFILE,
            double_spawn: &double_spawn,
            target_runner: &target_runner
        };

        let test_filter_builder = TestFilterBuilder::default_set(RunIgnored::Default);

        let cli_configs: Vec<String> = Vec::new();
        let cargo_configs = CargoConfigs::new(cli_configs.into_iter()).map_err(|error| TestRunError::Temp)?;

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
        ).map_err(|error| TestRunError::Temp)?;
        
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
            .map_err(|error| TestRunError::Temp)?;

        runner.execute(|test_event| {
            eprintln!("{:?}", test_event);
        }).map_err(|error| TestRunError::Temp)?;

        Ok(())
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

        let test_binaries = BinaryList::from_messages(Cursor::new(output.stdout), graph, build_platforms.clone())
            .map_err(|error| {
                eprintln!("{:?}", error);
                TestRunError::Io(std::io::Error::new(std::io::ErrorKind::AddrInUse, ""))
        })?;

        Ok(test_binaries)
    }
}
