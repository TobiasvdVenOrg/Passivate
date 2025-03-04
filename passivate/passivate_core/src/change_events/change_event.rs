use crate::configuration::PassivateConfig;
use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
pub enum ChangeEvent {
    File,
    Configuration(PassivateConfig),
    Exit
}