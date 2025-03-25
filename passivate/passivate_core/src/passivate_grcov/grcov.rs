use std::{fs, io::Error as IoError, path::{Path, PathBuf}, process::Command, time::Duration};

use crate::{actors::Cancellation, coverage::{ComputeCoverage, CoverageError, CoverageStatus, NoProfrawFilesError, NoProfrawFilesKind}};
use crate::passivate_cargo::cargo_metadata;
use super::CovdirJson;

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
    fn compute_coverage(&self, cancellation: Cancellation) -> Result<CoverageStatus, CoverageError> {
        match get_profraw_count(&self.output_path) {
            Ok(0) => {
                let error = NoProfrawFilesError { 
                    expected_path: self.output_path.clone(), 
                    kind: NoProfrawFilesKind::NoProfrawFilesExist 
                };

                return Err(CoverageError::NoProfrawFiles(error))
            }
            Err(io_error) => {
                let error = NoProfrawFilesError { 
                    expected_path: self.output_path.clone(), 
                    kind: NoProfrawFilesKind::Io(io_error.kind()) 
                };

                return Err(CoverageError::NoProfrawFiles(error))
            },
            _ => { }
        };
        
        cancellation.check()?;

        let projects = cargo_metadata::projects(&self.workspace_path);

        let mut command = Command::new("grcov");

        command.current_dir(&self.workspace_path)
            .arg(&self.output_path)
            .arg("-s")
            .arg(".")
            .arg("--binary-path")
            .arg(&self.binary_path)
            .arg("-t")
            .arg("covdir,lcov")
            .arg("--branch")
            .arg("--ignore-not-existing")
            .arg("-o")
            .arg(&self.output_path);

        for project in projects {
            let keep = PathBuf::from(project.file_name().unwrap()).join("src").join("*");
            command.arg("--keep-only").arg(keep);
        }

        let mut grcov = command.spawn()
            .map_err(|e| CoverageError::GrcovNotInstalled(e.kind()))?;

        loop {
            std::thread::sleep(Duration::from_millis(100));

            if cancellation.is_cancelled() {
                let _ = grcov.kill();
                break;
            }

            let exited = grcov.try_wait().map_err(|e| CoverageError::FailedToGenerate(e.kind()))?;

            if exited.is_some() {
                break;
            }
        }

        let covdir_path = self.output_path.join("covdir");
        let json = fs::read_to_string(&covdir_path).map_err(|e| CoverageError::CovdirRead(e.kind()))?;
        let parsed: CovdirJson = parse_covdir(&json)?;

        Ok(CoverageStatus::Done(Box::new(parsed)))
    }

    fn clean_coverage_output(&self) -> Result<(), CoverageError> {
        if let Ok(false) = fs::exists(&self.output_path) {
            return Ok(());
        }
    
        remove_profraw_files(&self.output_path)
            .map_err(|e| CoverageError::CleanIncomplete(e.kind()))
    }
}

pub fn parse_covdir(json: &str) -> Result<CovdirJson, CoverageError> {
    serde_json::from_str(json).map_err(|e| CoverageError::CovdirParse(e.to_string()))
}

pub fn get_profraw_count(path: &Path) -> Result<i32, IoError> {
    let mut count = 0;

    for profraw in fs::read_dir(path)? {
        if let Some(extension) = profraw?.path().extension() {
            if extension == "profraw" {
                count += 1;
            }
        }
    }

    Ok(count)
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
