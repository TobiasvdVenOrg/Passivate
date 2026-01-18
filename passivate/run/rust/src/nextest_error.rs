use std::convert::Infallible;
use std::fmt::Display;

use nextest_filtering::errors::ParseSingleError;
use nextest_runner::errors::{
    CargoConfigError,
    CargoMetadataError,
    ConfigParseError,
    CreateBinaryListError,
    CreateTestListError,
    FromMessagesError,
    HostPlatformDetectError,
    ProfileNotFound,
    TestFilterBuilderError,
    TestRunnerBuildError,
    TestRunnerExecuteErrors
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NextestError
{
    HostPlatformDetect(#[from] HostPlatformDetectError),
    CargoMetadata(#[from] CargoMetadataError),
    ConfigParse(#[from] ConfigParseError),
    CreateBinaryList(#[from] CreateBinaryListError),
    FromMessages(#[from] FromMessagesError),
    ProfileNotFound(#[from] ProfileNotFound),
    FiltersetParse(#[from] ParseSingleError),
    UnknownFiltersetParse,
    TestFilterBuilder(#[from] TestFilterBuilderError),
    CargoConfig(#[from] CargoConfigError),
    CreateTestList(#[from] CreateTestListError),
    TestRunnerBuild(#[from] TestRunnerBuildError),
    TestRunnerExecute(#[from] TestRunnerExecuteErrors<Infallible>)
}

impl Display for NextestError
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        todo!()
    }
}
