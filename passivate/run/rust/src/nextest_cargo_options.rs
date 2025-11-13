use camino::Utf8PathBuf;
use nextest_runner::cargo_cli::CargoOptions;

#[bon::builder]
pub fn cargo_options(
    #[builder(default = Vec::new())] packages: Vec<String>,
    #[builder(default = false)] workspace: bool,
    #[builder(default = Vec::new())] exclude: Vec<String>,
    #[builder(default = false)] all: bool,
    #[builder(default = false)] lib: bool,
    #[builder(default = Vec::new())] bin: Vec<String>,
    #[builder(default = false)] bins: bool,
    #[builder(default = Vec::new())] example: Vec<String>,
    #[builder(default = false)] examples: bool,
    #[builder(default = Vec::new())] test: Vec<String>,
    #[builder(default = false)] tests: bool,
    #[builder(default = Vec::new())] bench: Vec<String>,
    #[builder(default = false)] benches: bool,
    #[builder(default = false)] all_targets: bool,
    #[builder(default = Vec::new())] features: Vec<String>,
    #[builder(default = false)] all_features: bool,
    #[builder(default = false)] no_default_features: bool,
    build_jobs: Option<String>,
    #[builder(default = false)] release: bool,
    cargo_profile: Option<String>,
    target: Option<String>,
    target_dir: Option<Utf8PathBuf>,
    #[builder(default = false)] unit_graph: bool,
    timings: Option<Option<String>>,
    #[builder(default = false)] frozen: bool,
    #[builder(default = false)] locked: bool,
    #[builder(default = false)] offline: bool,
    #[builder(default = 0)] cargo_quiet: u8,
    #[builder(default = 0)] cargo_verbose: u8,
    #[builder(default = false)] ignore_rust_version: bool,
    #[builder(default = false)] future_incompat_report: bool,
    #[builder(default = Vec::new())] config: Vec<String>,
    #[builder(default = Vec::new())] unstable_flags: Vec<String>) -> CargoOptions {
    CargoOptions { packages, workspace, exclude, all, lib, bin, bins, example, examples, test, tests, bench, benches, all_targets, features, all_features, no_default_features, build_jobs, release, cargo_profile, target, target_dir, unit_graph, timings, frozen, locked, offline, cargo_quiet, cargo_verbose, ignore_rust_version, future_incompat_report, config, unstable_flags }
}
