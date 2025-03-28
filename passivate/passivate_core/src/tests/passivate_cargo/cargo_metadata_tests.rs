use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use crate::test_helpers::builder::cargo_builder;
use crate::passivate_cargo::cargo_workspace;
use crate::passivate_cargo::cargo_workspace_errors::CargoWorkspaceError;
use galvanic_assert::assert_that;
use galvanic_assert::has_structure;
use galvanic_assert::matchers::eq;
use galvanic_assert::structure;
use galvanic_assert::matchers::collection::*;

#[test]
pub fn query_single_project() {
    let workspace_path = cargo_builder()
        .with_workspace("simple_project").get_workspace_path();

    let projects = cargo_workspace::projects(&workspace_path).unwrap();

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn query_projects_in_workspace() {
    let workspace_path = cargo_builder()
        .with_workspace("simple_workspace").get_workspace_path();

    let projects = cargo_workspace::projects(&workspace_path).unwrap();

    let project_a_path = fs::canonicalize(workspace_path.join("project_a")).unwrap();
    let project_b_path = fs::canonicalize(workspace_path.join("project_b")).unwrap();

    assert_that!(&projects, contains_in_order(vec![project_a_path, project_b_path]));
}

#[test]
pub fn query_with_full_cargo_toml_path() {
    let workspace_path = cargo_builder()
        .with_workspace("simple_project").get_workspace_path();

    let projects = cargo_workspace::projects(&workspace_path.join("Cargo.toml")).unwrap();

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn cargo_toml_file_that_is_lower_case_is_user_error() {
    let workspace_path = cargo_builder()
        .with_workspace("incorrect_toml_casing").get_workspace_path();

    let result = cargo_workspace::projects(&workspace_path).unwrap_err();

    assert_that!(&result, has_structure!(CargoWorkspaceError::IncorrectTomlCasing {
        path: eq(PathBuf::from("\\\\?\\F:\\Projects\\Passivate\\test_data\\incorrect_toml_casing")),
        found: eq(OsString::from("cargo.toml"))
    }));
}