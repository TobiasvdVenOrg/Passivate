use std::{fs, path::{Path, PathBuf}, process::Command};
use crate::coverage::ComputeCoverage;
use crate::coverage::CoverageStatus;

pub struct GrcovComputeCoverage {

}

impl ComputeCoverage for GrcovComputeCoverage {
    fn compute_coverage() -> CoverageStatus {
        todo!()
    }
}

pub fn grcov(working_dir: &Path, profraw_dir: &Path, binary_path: &Path, output_dir: &Path) -> PathBuf {
    let lcov_info_path = output_dir.join("lcov.info");

    let _grcov = Command::new("grcov")
            .current_dir(working_dir)
            .arg(profraw_dir)
            .arg("-s")
            .arg(".")
            .arg("--binary-path")
            .arg(binary_path)
            .arg("-t")
            .arg("lcov")
            .arg("--branch")
            .arg("--ignore-not-existing")
            .arg("-o")
            .arg(&lcov_info_path)
            .spawn()
            .unwrap()
            .wait();

        lcov_info_path.to_path_buf()
}

pub fn remove_profraw_files(profraw_dir: &Path) -> Result<(), std::io::Error> {
    if !fs::exists(profraw_dir)? {
        return Ok(())
    }

    for profraw in fs::read_dir(profraw_dir)?.flatten() {      
        if let Some(extension) = profraw.path().extension() {
            if extension == "profraw" {
                fs::remove_file(profraw.path())?;
            }
        }
    }

    Ok(())
}
