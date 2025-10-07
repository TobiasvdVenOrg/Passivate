
pub mod hyp_id;



#[macro_export]
macro_rules! test_id {
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

#[macro_export]
macro_rules! test_name {
    () => {
        $crate::hyp_id::HypId::try_from($crate::test_id!().as_ref()).unwrap().get_name(&$crate::hyp_id::HypNameStrategy::Default).to_string()
    };
}
