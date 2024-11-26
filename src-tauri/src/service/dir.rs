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

use std::fs::DirBuilder;
use std::path::PathBuf;

use std::{
    fs::{self, ReadDir},
    io::Error,
    path::Path,
};

pub struct Directories {}

const IDENTIFIER: &str = "com.ardeck.studio";

impl Directories {
    #[cfg(feature = "portable")]
    pub fn get_config_dir() -> PathBuf {
        PathBuf::from("./").join("config")
    }

    #[cfg(not(feature = "portable"))]
    pub fn get_config_dir() -> std::io::Result<PathBuf> {
        let path = match dirs::config_dir() {
            Some(p) => p,
            None => return Err(Error::new(std::io::ErrorKind::NotFound, "Config directory not found")),
        };

        Ok(path.join(IDENTIFIER).join("config"))
    }

    pub fn init<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        fs::create_dir_all(path)
    }

    pub fn get<P: AsRef<Path>>(path: P) -> Result<ReadDir, Error> {
        fs::read_dir(path)
    }

    pub fn get_or_init<P: AsRef<Path>>(path: P) -> Result<ReadDir, Error> {
        let dir = Self::get(&path);

        match dir {
            Ok(dir) => Ok(dir),
            Err(_) => {
                let new_dir = Self::init(&path);

                match new_dir {
                    Ok(_) => Self::get(path),
                    Err(error) => Err(error),
                }
            }
        }
    }
}
