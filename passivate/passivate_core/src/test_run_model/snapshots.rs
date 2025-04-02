use std::{fs::File, io::Error as IoError, path::{Path, PathBuf}};

use epaint::ColorImage;

use super::SingleTest;
use thiserror::Error;

pub struct Snapshot {
    pub current: Option<Result<ColorImage, SnapshotError>>,
    pub new: Option<Result<ColorImage, SnapshotError>>
}

#[derive(Clone)]
pub struct Snapshots {
    pub snapshot_directory: PathBuf
}

#[derive(Error, Debug)]
pub enum SnapshotError {
    #[error("snapshot format is not supported: {color_type:?}")]
    Unsupported {
        color_type: png::ColorType
    },

    #[error("snapshot data did not match expected size")]
    InvalidData,

    #[error("io error occurred loading snapshot:\n{error}\n{path}")]
    Io {
        error: IoError,
        path: PathBuf
    }
}

impl Snapshots {
    pub fn new(snapshot_directory: PathBuf) -> Self {
        Self { snapshot_directory }
    }

    pub fn from_test(&self, single_test: &SingleTest) -> Snapshot {
        let current = self.from_file(PathBuf::from(&single_test.name).with_extension("png"));
        let new = self.from_file(PathBuf::from(&single_test.name).with_extension("new.png"));
        
        Snapshot { current, new }
    }

    pub fn from_file(&self, file: PathBuf) -> Option<Result<ColorImage, SnapshotError>> {
        let path = self.snapshot_directory.join(file);

        if let Some(open_result) = Self::open_file(&path) {
            return match open_result {
                Ok(file) => Some(Self::decode_image(file)),
                Err(error) => Some(Err(error))
            }
        }

        None
    }

    fn decode_image(file: File) -> Result<ColorImage, SnapshotError> {
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer).unwrap();

        let width = info.width as usize;
        let height = info.height as usize;
        let dimensions = [width, height];
        let buffer_size = info.buffer_size();
        let data = &buffer[..buffer_size];
        let color_type = reader.output_color_type().0;

        match color_type {
            png::ColorType::Grayscale => {
                if width * height != data.len() {
                    return Err(SnapshotError::InvalidData)
                }

                Ok(ColorImage::from_gray(dimensions, data))
            },
            png::ColorType::Rgb => {
                if width * height * 3 != data.len() {
                    return Err(SnapshotError::InvalidData)
                }

                Ok(ColorImage::from_rgb(dimensions, data))
            }
            png::ColorType::Indexed => Err(SnapshotError::Unsupported { color_type }),
            png::ColorType::GrayscaleAlpha => Err(SnapshotError::Unsupported { color_type }),
            png::ColorType::Rgba => {
                if width * height * 4 != data.len() {
                    return Err(SnapshotError::InvalidData)
                }

                Ok(ColorImage::from_rgba_unmultiplied(dimensions, data))
            }
        }
    }

    fn open_file(path: &Path) -> Option<Result<File, SnapshotError>> {
        let open = File::open(path);

        match open {
            Ok(file) => Some(Ok(file)),
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    return None
                }

                Some(Err(SnapshotError::Io { error, path: path.to_path_buf() }))
            },
        }
    }
}