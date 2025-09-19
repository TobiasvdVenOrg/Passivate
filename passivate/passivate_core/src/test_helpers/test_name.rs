
pub fn test_name(function_name: &str) -> String
{
    function_name.split("::").skip(2).collect::<Vec<_>>().join("-")
}
