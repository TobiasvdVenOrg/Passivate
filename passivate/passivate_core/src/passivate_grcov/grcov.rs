use std::{fs, io::Error as IoError, path::{Path, PathBuf}, process::Command};
use crate::coverage::{ComputeCoverage, CoverageError, CoverageStatus};

pub struct Grcov {
    workspace_path: PathBuf,
    output_path: PathBuf,
    binary_path: PathBuf
}

impl Grcov {
    pub fn new(workspace_path: &Path, output_path: &Path, binary_path: &Path) -> Self {
        Self { workspace_path: workspace_path.to_path_buf(), output_path: output_path.to_path_buf(), binary_path: binary_path.to_path_buf() }
    }
}

impl ComputeCoverage for Grcov {
    fn compute_coverage(&self) -> Result<CoverageStatus, CoverageError> {
        // Omit *.info extension, since it messes up grcov when a .info file already exists in the target directory
        let lcov_info_path = self.output_path.join("lcov");

        let mut grcov = Command::new("grcov")
            .current_dir(&self.workspace_path)
            .arg(&self.output_path)
            .arg("-s")
            .arg(".")
            .arg("--binary-path")
            .arg(&self.binary_path)
            .arg("-t")
            .arg("lcov")
            .arg("--branch")
            .arg("--ignore-not-existing")
            .arg("-o")
            .arg(&lcov_info_path)
            .spawn()
            .map_err(|e| CoverageError::GrcovNotInstalled(e.kind()))?;

        grcov.wait().map_err(|e| CoverageError::FailedToGenerate(e.kind()))?;

        Ok(CoverageStatus::Disabled)
    }

    fn clean_coverage_output(&self) -> Result<(), CoverageError> {
        if let Ok(false) = fs::exists(&self.output_path) {
            return Ok(());
        }
    
        remove_profraw_files(&self.output_path)
            .map_err(|e| CoverageError::CleanIncomplete(e.kind()))
    }
}

fn remove_profraw_files(directory: &Path) -> Result<(), IoError> {
    for profraw in fs::read_dir(directory)?.flatten() {      
        if let Some(extension) = profraw.path().extension() {
            if extension == "profraw" {
                fs::remove_file(profraw.path())?;
            }
        }
    }

    Ok(())
}
