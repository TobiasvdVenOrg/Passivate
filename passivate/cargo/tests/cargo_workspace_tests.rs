use std::ffi::OsString;

use camino::Utf8PathBuf;
use galvanic_assert::{assert_that, matchers::{collection::contains_in_order, eq}, structure};
use passivate_cargo::{cargo_workspace, cargo_workspace_errors::CargoWorkspaceError};
use passivate_testing::path_resolution::get_default_workspace_path;

#[test]
pub fn query_single_project()
{
    let workspace_path = get_default_workspace_path("simple_project");

    let projects = cargo_workspace::projects(&workspace_path).unwrap();

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn query_projects_in_workspace()
{
    let workspace_path = get_default_workspace_path("simple_workspace");

    let projects = cargo_workspace::projects(&workspace_path).unwrap();

    let project_a_path = Utf8PathBuf::from_path_buf(dunce::canonicalize(workspace_path.join("project_a")).unwrap()).unwrap();
    let project_b_path = Utf8PathBuf::from_path_buf(dunce::canonicalize(workspace_path.join("project_b")).unwrap()).unwrap();

    assert_that!(&projects, contains_in_order(vec![project_a_path, project_b_path]));
}

#[test]
pub fn query_with_full_cargo_toml_path()
{
    let workspace_path = get_default_workspace_path("simple_project");

    let projects = cargo_workspace::projects(&workspace_path.join("Cargo.toml")).unwrap();

    assert_that!(&projects, contains_in_order(vec![workspace_path]));
}

#[test]
pub fn cargo_toml_file_that_is_lower_case_is_user_error()
{
    let workspace_path = get_default_workspace_path("incorrect_toml_casing");

    let result = cargo_workspace::projects(&workspace_path).unwrap_err();

    assert_that!(
        &result,
        structure!(CargoWorkspaceError::IncorrectTomlCasing {
            path: eq(workspace_path.to_path_buf()),
            found: eq(OsString::from("cargo.toml"))
        })
    );
}