use std::convert::Infallible;
use std::fmt::Display;

use cargo_nextest::ExpectedError;
use nextest_filtering::errors::ParseSingleError;
use nextest_runner::errors::{
    CargoConfigError,
    ConfigParseError,
    CreateTestListError,
    FromMessagesError,
    HostPlatformDetectError,
    ProfileNotFound,
    TestFilterBuildError,
    TestRunnerBuildError,
    TestRunnerExecuteErrors
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NextestError
{
    HostPlatformDetect(#[from] HostPlatformDetectError),
    ConfigParse(#[from] ConfigParseError),
    FromMessages(#[from] FromMessagesError),
    ProfileNotFound(#[from] ProfileNotFound),
    FiltersetParse(#[from] ParseSingleError),
    UnknownFiltersetParse,
    CargoConfig(#[from] CargoConfigError),
    CreateTestList(#[from] CreateTestListError),
    TestFilterBuild(#[from] TestFilterBuildError),
    TestRunnerBuild(#[from] TestRunnerBuildError),
    TestRunnerExecute(#[from] TestRunnerExecuteErrors<Infallible>),
    Expected(#[from] ExpectedError)
}

impl Display for NextestError
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        todo!()
    }
}
