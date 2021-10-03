use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView, RgbaImage};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct ImageData {
    pub image: RgbaImage,
    pub width: u32,
    pub height: u32,
}

pub struct ResourceLoader {
    root_path: PathBuf,
}

impl ResourceLoader {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<ResourceLoader, Error> {
        let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;
        Ok(ResourceLoader {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path, resource_name)
        )?;
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    pub fn load_image(&self, resource_name: &str) -> ImageData {
        let path = resource_name_to_path(&self.root_path, resource_name);
        match image::open(&path) {
            Err(err) => panic!("Could not load image {}: {}", path.as_os_str().to_str().unwrap(), err),
            Ok(img) => {
                println!("Dimensions of image are {:?}", img.dimensions());
                let (width, height) = img.dimensions();
                let flipped = img.rotate180().fliph();

                let flipped = match flipped {
                    DynamicImage::ImageRgba8(flipped) => flipped,
                    flipped => flipped.to_rgba8()
                };
                return ImageData {
                    image: flipped,
                    width,
                    height,
                };
            }
        }
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}

