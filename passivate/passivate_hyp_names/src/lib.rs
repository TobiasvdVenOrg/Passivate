
pub mod hyp_id;
pub mod hyp_name_strategy;

#[macro_export]
macro_rules! test_name {
    () => {
        if !stdext::function_name!().contains(env!("CARGO_PKG_NAME"))
        {
            format!("{}::{}", env!("CARGO_PKG_NAME"), stdext::function_name!())
        }
        else
        {
            stdext::function_name!().to_string()
        }
    };
}