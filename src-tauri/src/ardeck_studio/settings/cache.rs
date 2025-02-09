/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 Project Ardeck

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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheInfo {
    pub path: PathBuf,
    pub dirty: bool,
    pub data: String,
}

pub struct Cache {
    inner: Vec<CacheInfo>,
}

impl Cache {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn is_dirty(&self, path: &PathBuf) -> bool { // 前回からデータが変更されていないか、そもそもデータが存在しなければfalse
        self.inner.iter().any(|c| c.path == *path && c.dirty)
    }

    pub fn set_dirty(&mut self, path: &PathBuf, dirty: bool) { // dirtyの状態を変更する
        self.inner
            .iter_mut()
            .find(|c| c.path == *path)
            .map(|c| c.dirty = dirty);
    }

    pub fn mark_dirty(&mut self, path: &PathBuf) {
        self.inner
            .iter_mut()
            .find(|c| c.path == *path)
            .map(|c| c.dirty = true);
    }

    pub fn remove(&mut self, path: &PathBuf) {
        self.inner.retain(|c| c.path != *path);
    }

    

    pub fn get(&self, path: &PathBuf) -> Option<CacheInfo> {
        self.inner.iter().find(|c| c.path == *path).cloned()
    }

    pub fn get_data(&self, path: &PathBuf) -> Option<String> {
        self.inner
            .iter()
            .find(|c| c.path == *path)
            .map(|c| c.data.clone())
    }

    pub fn add(&mut self, path: PathBuf, data: String, dirty: bool) {
        self.inner.push(CacheInfo {
            path,
            dirty,
            data,
        });
    }

    pub fn update_data(&mut self, path: &PathBuf, data: String) { // ファイルが変更された後に読み込まれた後、データを更新し、dirtyをfalseにする
        self.inner
            .iter_mut()
            .find(|c| c.path == *path)
            .map(|c| {
                c.dirty = false;
                c.data = data;
            });
    }
}
