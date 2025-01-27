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

use std::{fs::File, path::{Path, PathBuf}};

use super::dir::Directories;

pub struct Files;
impl Files {
    fn validate_path<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
        let path = path.as_ref();
        let canonical = path.canonicalize()?;

        if canonical.starts_with(Directories::get_settings_dir()?) {
            Ok(canonical)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Permission denied"))
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        File::open(Self::validate_path(path)?)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        File::create(Self::validate_path(path)?)
    }

    pub fn remove<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        std::fs::remove_file(Self::validate_path(path)?)
    }
}
