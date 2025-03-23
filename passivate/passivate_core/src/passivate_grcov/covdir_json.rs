use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovdirJson {
    pub children: Option<HashMap<String, CovdirJson>>,
    pub coverage_percent: f64,
    pub lines_covered: i64,
    pub lines_missed: i64,
    pub lines_total: i64,
    pub name: String,
}
