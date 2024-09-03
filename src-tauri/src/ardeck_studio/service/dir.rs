use std::{
    f32::consts::E, fs::{self, DirBuilder, ReadDir}, io::{
        Error, ErrorKind
    }, path::Path
};

pub struct Directories {}

impl Directories {
    pub fn init(path: &Path) -> Result<(), Error> {
        DirBuilder::new().recursive(false).create(path)
    }

    pub fn get(path: &Path) -> Result<ReadDir, Error> {
        fs::read_dir(path)
    }

    pub fn get_or_init(path: &Path) -> Result<ReadDir, Error> {
        let dir = Self::get(path);

        match dir {
            Ok(dir) => Ok(dir),
            Err(_) => {
                let new_dir = Self::init(path);

                match new_dir {
                    Ok(_) => Self::get(path),
                    Err(error) => Err(error),
                }
            }
        }
    }
}
