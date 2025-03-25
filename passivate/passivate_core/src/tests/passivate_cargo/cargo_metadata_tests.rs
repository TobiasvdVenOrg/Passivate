use std::fs;

use crate::test_helpers::builder::cargo_builder;
use crate::passivate_cargo::cargo_metadata;
use galvanic_assert::assert_that;
use galvanic_assert::matchers::collection::*;

#[test]
pub fn query_single_project() {
    let workspace_path = cargo_builder()
        .with_workspace("simple_project").get_workspace_path();

    let projects = cargo_metadata::projects(&workspace_path);

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn query_projects_in_workspace() {
    let workspace_path = cargo_builder()
        .with_workspace("simple_workspace").get_workspace_path();

    let projects = cargo_metadata::projects(&workspace_path);

    let project_a_path = fs::canonicalize(workspace_path.join("project_a")).unwrap();
    let project_b_path = fs::canonicalize(workspace_path.join("project_b")).unwrap();

    assert_that!(&projects, contains_in_order(vec![project_a_path, project_b_path]));
}