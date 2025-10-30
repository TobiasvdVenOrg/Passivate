
pub mod hyp_id;



#[macro_export]
macro_rules! test_id {
    () => {
        {
            let package_name = env!("CARGO_PKG_NAME");
            let function_name = stdext::function_name!();
            let first = function_name.split("::").next().expect(format!("failed to find first part of test function: {}", function_name).as_str());

            if first == package_name
            {               
                let without_package = function_name.strip_prefix(package_name).expect(format!("failed to strip '{}' from '{}", package_name, function_name).as_str());
                let strip_separator = without_package.strip_prefix("::").expect(format!("failed to strip '::' from '{}'", without_package).as_str());

                $crate::hyp_id::HypId::new(package_name, strip_separator).unwrap_or_else(|error| panic!("{:?}", error))
            }
            else
            {
                $crate::hyp_id::HypId::new(package_name, function_name).unwrap_or_else(|error| panic!("{:?}", error))
            }
        }
    };
}

#[macro_export]
macro_rules! test_name {
    () => {
        $crate::test_id!().get_name(&$crate::hyp_id::HypNameStrategy::Default).to_string()
    };
}
