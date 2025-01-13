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

use super::SettingsStore;

pub struct CacheInfo<T> {
    pub path: PathBuf,
    pub dirty: bool,
    pub data: T,
}

pub struct Cache<Impl, T>
where
    Impl: SettingsStore<T>,
{
    inner: Vec<CacheInfo<T>>,
    _marker: std::marker::PhantomData<Impl>,
}

impl<Impl, T> Cache<Impl, T>
where
    Impl: SettingsStore<T>,
{
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_dirty(&self, path: &PathBuf) -> bool {
        self.inner.iter().any(|c| c.path == *path)
    }

    pub fn get(&self, path: &PathBuf) -> Option<&T> {
        self.inner.iter().find(|c| c.path == *path).map(|c| &c.data)
    }

    pub fn set(&mut self, path: &PathBuf, data: T) {
        self.inner.push(CacheInfo {
            path: path.clone(),
            dirty: true,
            data,
        });
    }
}
