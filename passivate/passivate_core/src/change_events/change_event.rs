use enum_as_inner::EnumAsInner;

use crate::{configuration::ConfigurationEvent, test_run_model::TestId};

#[derive(Clone)]
#[derive(EnumAsInner)]
pub enum ChangeEvent {
    File,
    Configuration(ConfigurationEvent),
    SingleTest {
        id: TestId,
        update_snapshots: bool
    },
    PinTest {
        id: TestId
    },
    ClearPinnedTests
}
