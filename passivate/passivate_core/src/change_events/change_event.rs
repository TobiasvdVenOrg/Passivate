use enum_as_inner::EnumAsInner;

use crate::{configuration::PassivateConfig, test_run_model::TestId};

#[derive(Clone)]
#[derive(EnumAsInner)]
pub enum ChangeEvent {
    File,
    Configuration(PassivateConfig),
    SingleTest {
        id: TestId,
        update_snapshots: bool
    }
}
