/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 project-ardeck

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or 
(at your option) any later version.

This program is distributed in the hope that it will be useful, 
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the 
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::path::PathBuf;

use std::{
    f32::consts::E, fs::{self, DirBuilder, ReadDir}, io::{
        Error, ErrorKind
    }, path::Path
};

pub struct Directories {}

impl Directories {
    #[cfg(feature="portable")]
    pub fn get_config_dir() -> PathBuf {
        PathBuf::from("./")
    }

    #[cfg(not(feature="portable"))]
    pub fn get_config_dir() -> PathBuf {
        dirs::config_dir().unwrap()
    }

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
