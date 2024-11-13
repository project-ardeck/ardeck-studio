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

use std::{fs::File, path::Path};

pub struct Files;
impl Files {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        File::open(path)
    }

    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        File::create(path)
    }

    pub fn remove<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }

    pub fn open_or_create<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        let is_exist = path.as_ref().try_exists().unwrap_or(false);
        if !is_exist {
            Self::create(&path)?;
        }
        Self::open(path)
    }
}
