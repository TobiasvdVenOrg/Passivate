use std::ffi::OsString;
use std::fs;

use galvanic_assert::matchers::collection::*;
use galvanic_assert::matchers::eq;
use galvanic_assert::{assert_that, structure};

use crate::passivate_cargo::cargo_workspace;
use crate::passivate_cargo::cargo_workspace_errors::CargoWorkspaceError;
use crate::test_helpers::test_run_setup;

#[test]
pub fn query_single_project()
{
    let workspace_path = test_run_setup::get_default_workspace_path("simple_project");

    let projects = cargo_workspace::projects(&workspace_path).unwrap();

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn query_projects_in_workspace()
{
    let workspace_path = test_run_setup::get_default_workspace_path("simple_workspace");

    let projects = cargo_workspace::projects(&workspace_path).unwrap();

    let project_a_path = fs::canonicalize(workspace_path.join("project_a")).unwrap();
    let project_b_path = fs::canonicalize(workspace_path.join("project_b")).unwrap();

    assert_that!(&projects, contains_in_order(vec![project_a_path, project_b_path]));
}

#[test]
pub fn query_with_full_cargo_toml_path()
{
    let workspace_path = test_run_setup::get_default_workspace_path("simple_project");

    let projects = cargo_workspace::projects(&workspace_path.join("Cargo.toml")).unwrap();

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn cargo_toml_file_that_is_lower_case_is_user_error()
{
    let workspace_path = test_run_setup::get_default_workspace_path("incorrect_toml_casing");

    let result = cargo_workspace::projects(&workspace_path).unwrap_err();

    assert_that!(
        &result,
        structure!(CargoWorkspaceError::IncorrectTomlCasing {
            path: eq(workspace_path.to_path_buf()),
            found: eq(OsString::from("cargo.toml"))
        })
    );
}
