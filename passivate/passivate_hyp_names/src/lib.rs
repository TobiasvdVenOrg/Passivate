
pub mod hyp_id;



#[macro_export]
macro_rules! test_id {
    () => {
        if stdext::function_name!().contains(env!("CARGO_PKG_NAME"))
        {
            $crate::hyp_id::HypId::new(env!("CARGO_PKG_NAME"), stdext::function_name!().strip_prefix(env!("CARGO_PKG_NAME")).unwrap().strip_prefix("::").unwrap()).map_err(|error| panic!("{:?}", error)).unwrap()
        }
        else
        {
            $crate::hyp_id::HypId::new(env!("CARGO_PKG_NAME"), stdext::function_name!()).map_err(|error| panic!("{:?}", error)).unwrap()
        }
    };
}

#[macro_export]
macro_rules! test_name {
    () => {
        $crate::test_id!().get_name(&$crate::hyp_id::HypNameStrategy::Default).to_string()
    };
}
