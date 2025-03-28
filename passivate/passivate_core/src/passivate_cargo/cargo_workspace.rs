use std::{fs, path::{Path, PathBuf}};

use super::CargoWorkspaceError;


pub fn projects(workspace: &Path) -> Result<Vec<PathBuf>, CargoWorkspaceError> {
    let mut toml = workspace.to_path_buf();

    if toml.is_dir() {
        if let Some(toml_name) = fs::read_dir(toml)?
            .flatten()
            .find(|f| f.file_name().eq_ignore_ascii_case("cargo.toml"))
            .map(|dir| dir.file_name()) {
            
            if toml_name != "Cargo.toml" {
                return Err(CargoWorkspaceError::IncorrectTomlCasing { path: workspace.to_path_buf(), found: toml_name })
            }

            toml = workspace.join(toml_name);
        } else {
            return Err(CargoWorkspaceError::TomlNotFound(workspace.to_path_buf()));
        }
    }

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(toml)
        .no_deps()
        .exec()?;

    let project_names = metadata.packages.iter()
        .filter(|package| {
            metadata.workspace_members.iter().any(|member| *member == package.id)
        })
        .map(|package| package.manifest_path.clone().into_std_path_buf())
        .filter_map(|path| path.parent().map(|directory| directory.to_path_buf()))
        .collect();

    Ok(project_names)
}
