pub mod snapshot_handles;

use std::fs::File;
use std::io::Error as IoError;

use camino::{Utf8Path, Utf8PathBuf};
use epaint::ColorImage;
use passivate_hyp_names::hyp_id::{HypId, HypNameStrategy};
use png::DecodingError;
use thiserror::Error;

pub struct Snapshot
{
    pub current: Option<Result<ColorImage, SnapshotError>>,
    pub new: Option<Result<ColorImage, SnapshotError>>
}

#[derive(Clone, Debug)]
pub struct Snapshots
{
    pub snapshot_directory: Utf8PathBuf
}

#[derive(Error, Debug)]
pub enum SnapshotError
{
    #[error("snapshot format '{color_type:?}' is not supported: {path}")]
    Unsupported
    {
        color_type: png::ColorType,
        path: Utf8PathBuf
    },

    #[error("snapshot data did not match expected size: {path}")]
    InvalidData
    {
        path: Utf8PathBuf
    },

    #[error("io error occurred loading snapshot:\n{error}\n{path}")]
    Io
    {
        error: IoError, path: Utf8PathBuf
    },

    #[error("decoding error {error} in snapshot: {path}")]
    Decoding
    {
        error: DecodingError,
        path: Utf8PathBuf
    }
}

impl Snapshots
{
    pub fn new(snapshot_directory: Utf8PathBuf) -> Self
    {
        Self { snapshot_directory }
    }

    pub fn from_hyp(&self, hyp_id: &HypId) -> Snapshot
    {
        let file_name = hyp_id.get_name(&HypNameStrategy::Default);
        let current = self.from_file(Utf8PathBuf::from(&file_name).with_extension("png"));
        let new = self.from_file(Utf8PathBuf::from(&file_name).with_extension("new.png"));

        Snapshot { current, new }
    }

    pub fn from_file(&self, file_path: Utf8PathBuf) -> Option<Result<ColorImage, SnapshotError>>
    {
        let path = self.snapshot_directory.join(&file_path);

        if let Some(open_result) = Self::open_file(&path)
        {
            return match open_result
            {
                Ok(file) => Some(Self::decode_image(file, file_path)),
                Err(error) => Some(Err(error))
            };
        }

        None
    }

    fn decode_image(file: File, path: Utf8PathBuf) -> Result<ColorImage, SnapshotError>
    {
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info().map_err(|e| SnapshotError::Decoding { error: e, path: path.clone() })?;
        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer).unwrap();

        let width = info.width as usize;
        let height = info.height as usize;
        let dimensions = [width, height];
        let buffer_size = info.buffer_size();
        let data = &buffer[.. buffer_size];
        let color_type = reader.output_color_type().0;

        match color_type
        {
            png::ColorType::Grayscale =>
            {
                if width * height != data.len()
                {
                    return Err(SnapshotError::InvalidData { path });
                }

                Ok(ColorImage::from_gray(dimensions, data))
            }
            png::ColorType::Rgb =>
            {
                if width * height * 3 != data.len()
                {
                    return Err(SnapshotError::InvalidData { path });
                }

                Ok(ColorImage::from_rgb(dimensions, data))
            }
            png::ColorType::Indexed => Err(SnapshotError::Unsupported { color_type, path }),
            png::ColorType::GrayscaleAlpha => Err(SnapshotError::Unsupported { color_type, path }),
            png::ColorType::Rgba =>
            {
                if width * height * 4 != data.len()
                {
                    return Err(SnapshotError::InvalidData { path });
                }

                Ok(ColorImage::from_rgba_unmultiplied(dimensions, data))
            }
        }
    }

    fn open_file(path: &Utf8Path) -> Option<Result<File, SnapshotError>>
    {
        let open = File::open(path);

        match open
        {
            Ok(file) => Some(Ok(file)),
            Err(error) =>
            {
                if error.kind() == std::io::ErrorKind::NotFound
                {
                    return None;
                }

                Some(Err(SnapshotError::Io { error, path: path.to_path_buf() }))
            }
        }
    }
}
