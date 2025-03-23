use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovdirJson {
    pub children: Children,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub src: Src,
    pub tests: Tests,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Src {
    pub children: Children2,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    #[serde(rename = "lib.rs")]
    pub lib_rs: LibRs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibRs {
    pub coverage: Vec<i64>,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tests {
    pub children: Children3,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children3 {
    #[serde(rename = "add_tests.rs")]
    pub add_tests_rs: AddTestsRs,
    #[serde(rename = "multiply_tests.rs")]
    pub multiply_tests_rs: MultiplyTestsRs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTestsRs {
    pub coverage: Vec<i64>,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiplyTestsRs {
    pub coverage: Vec<i64>,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}
