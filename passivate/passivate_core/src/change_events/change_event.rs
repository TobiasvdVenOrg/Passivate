use enum_as_inner::EnumAsInner;

use crate::configuration::PassivateConfig;

#[derive(Clone)]
#[derive(EnumAsInner)]
pub enum ChangeEvent {
    File,
    Configuration(PassivateConfig)
}
