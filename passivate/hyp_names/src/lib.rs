pub mod crate_id;
pub mod hyp_id;
pub mod package_id;
pub use stdext;

#[macro_export]
macro_rules! test_id {
    () => {{
        let package_name = env!("CARGO_PKG_NAME");
        let crate_name = env!("CARGO_CRATE_NAME");
        let function_name = $crate::stdext::function_name!();

        let first = function_name
            .split("::")
            .next()
            .expect(format!("failed to find first part of test function: {}", function_name).as_str());

        if first == crate_name
        {
            let without_crate = function_name
                .strip_prefix(crate_name)
                .expect(format!("failed to strip '{}' from '{}", crate_name, function_name).as_str());
            let strip_separator = without_crate
                .strip_prefix("::")
                .expect(format!("failed to strip '::' from '{}'", without_crate).as_str());

            $crate::hyp_id::HypId::new(package_name, crate_name, strip_separator)
        }
        else
        {
            $crate::hyp_id::HypId::new(package_name, crate_name, function_name)
        }
    }};
}

#[macro_export]
macro_rules! test_name {
    () => {
        $crate::test_id!()
            .name($crate::hyp_id::HypNameStrategy::Default)
            .to_string()
    };
}
