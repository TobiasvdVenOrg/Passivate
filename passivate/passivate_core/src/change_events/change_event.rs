use enum_as_inner::EnumAsInner;

use crate::test_run_model::TestId;

#[derive(Clone, EnumAsInner)]
pub enum ChangeEvent
{
    DefaultRun,
    SingleTest
    {
        id: TestId,
        update_snapshots: bool
    },
    PinTest
    {
        id: TestId
    },
    ClearPinnedTests
}
