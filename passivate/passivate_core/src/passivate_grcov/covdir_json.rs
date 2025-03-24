use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovdirJson {
    pub children: Option<IndexMap<String, CovdirJson>>,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}
