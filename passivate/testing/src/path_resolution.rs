use std::{env, fs};

use camino::{Utf8Path, Utf8PathBuf};

pub fn test_output_path() -> Utf8PathBuf
{
    let temp_dir = Utf8PathBuf::from_path_buf(env::temp_dir()).expect("temp_dir was not UTF8");
    temp_dir.join("passivate").join("test_output")
}

pub fn test_data_path() -> Utf8PathBuf
{
    let test_data = env::var("PASSIVATE_TEST_DATA").expect("environment variable 'PASSIVATE_TEST_DATA'");
    Utf8PathBuf::from(test_data)
}

pub fn get_default_workspace_path<P>(workspace_path: P) -> Utf8PathBuf
where
    P: AsRef<Utf8Path>
{
    test_data_path().join(workspace_path)
}

pub fn copy_from_data_to_output<P>(relative_path: P) -> Result<Utf8PathBuf, std::io::Error>
where
    P: AsRef<Utf8Path>
{
    let from = test_data_path().join(&relative_path);
    let to = test_output_path().join(&relative_path);

    let dir = to.parent().expect("expected parent directory in copy_from_data_to_output");

    if !fs::exists(dir)?
    {
        fs::create_dir_all(dir)?;
    }

    let _ = fs::copy(&from, &to)?;

    Ok(to)
}

pub fn clean_directory<P>(path: P)
where
    P: AsRef<Utf8Path>
{
    let path = path.as_ref();

    if fs::exists(path).expect("failed to check if path to clean exists")
    {
        eprintln!("cleaning: {:?}", path);

        fs::remove_dir_all(path).expect("failed to clean path");
    }
}
