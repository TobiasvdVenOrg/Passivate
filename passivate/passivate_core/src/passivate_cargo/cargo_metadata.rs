use std::path::{Path, PathBuf};


pub fn projects(workspace: &Path) -> Vec<PathBuf> {
    let toml = workspace.join("Cargo.toml");

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(toml)
        .no_deps()
        .exec()
        .unwrap();

    metadata.packages.iter()
        .filter(|package| {
            metadata.workspace_members.iter().any(|member| *member == package.id)
        })
        .map(|package| package.manifest_path.clone().into_std_path_buf())
        .filter_map(|path| path.parent().map(|directory| directory.to_path_buf()))
        .collect()
}
