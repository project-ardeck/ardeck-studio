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

use std::{
    fmt::Debug,
    io,
    path::{Path, PathBuf},
};

use cache::Cache;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
    sync::Mutex,
};

pub mod cache;
pub mod definitions;
pub mod tauri;

#[derive(Debug)]
pub enum SettingsStoreError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

pub trait SettingFile: Serialize + DeserializeOwned + Default + Clone + Send + Sync {
    fn name(&self) -> &'static str;
    fn dir(&self) -> PathBuf;
    fn file_path(&self) -> PathBuf {
        self.dir().join(format!("{}.json", self.name()))
    }
}

// TODO: saveしたらフラグを立て、loadするときに確認し、フラグが立っていなければ前回のデータをそのまま返すようなキャッシュ機能を作成する
// static CACHE: Lazy<Mutex<Cache<(), ()>>> = Lazy::new(|| Mutex::new(Cache::new()));
static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::new()));

pub trait SettingsStore:
    Serialize + DeserializeOwned + Default + Clone + Send + Sync + SettingFile + Debug
{
    async fn file_open<P: AsRef<Path>>(&self, path: P) -> Option<File> {
        match File::open(path).await {
            Ok(file) => Some(file),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    return None;
                }
                _ => return None,
            },
        }
    }

    /// キャッシュを無視してファイルから直接読み込む。
    /// ここで読み込まれたデータもキャッシュに保存される。
    async fn load_force(&mut self) -> Option<Self> {
        let file_path = self.file_path();
        let mut file = self.file_open(&file_path).await;

        // ファイルが存在しない場合は空のデータをセーブする。
        if file.is_none() {
            self.save().await;

            return Some(Self::default());
        }

        let mut reader = BufReader::new(file.unwrap());

        let mut file_str = String::new();

        reader.read_to_string(&mut file_str).await.unwrap();
        Some(serde_json::from_str(&file_str).unwrap())
    }

    /// ファイルを読み込みます。
    /// キャッシュが存在する場合はキャッシュをそのまま返し、キャッシュがファイルより古い場合はファイルを読み込みます。
    async fn load(&mut self) -> Option<Self> {
        // TODO: cache
        // self.inner.iter().find(|a| a.0.)
        let file_path = self.file_path();
        let mut file: Option<File> = None;
        let mut file_str = String::new();

        if CACHE.lock().await.get(&file_path).is_none() {
            // キャッシュが存在しない場合は、新たに読み込んでキャッシュを作る
            log::info!("File load: {}", file_path.display());
            log::trace!("load(cache is none): {}", file_path.display());

            file = self.file_open(&file_path).await;

            // ファイルが存在しない場合は空のデータをセーブする。
            if file.is_none() {
                self.save().await;

                return Some(Self::default());
            }

            file.unwrap().read_to_string(&mut file_str).await.unwrap();

            // キャッシュを作る
            CACHE
                .lock()
                .await
                .add(file_path.clone(), file_str.clone(), false);
        } else if CACHE.lock().await.is_dirty(&file_path) {
            // キャッシュがファイルより古い可能性があるときは、新たに読み込んでキャッシュも更新する
            log::trace!("load(cache is dirty): {}", file_path.display());

            file = self.file_open(&file_path).await;

            if file.is_none() {
                return Some(Self::default());
            }

            file.unwrap().read_to_string(&mut file_str).await.unwrap();

            // キャッシュを更新する
            CACHE.lock().await.update_data(&file_path, file_str.clone());
        } else {
            // キャッシュが存在する場合は、キャッシュを読み込む
            log::trace!("load(from cache): {}", file_path.display());

            let cache = CACHE.lock().await.get_data(&file_path);
            file_str = cache.unwrap();
        };

        match serde_json::from_str(&file_str) {
            Ok(setting) => {
                self.clone_from(&setting);
                return Some(setting);
            }
            Err(_) => return None,
        }
    }

    async fn save(&self) {
        // TODO: Error handling
        let path = self.file_path();
        log::info!("File save: {}", path.display());

        let file = std::fs::File::create(&path).unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).unwrap();

        CACHE.lock().await.mark_dirty(&path);
    }
}

#[macro_export]
macro_rules! setting {
    ($vis:vis type $name:ident = $t:ty;) => {
        use crate::ardeck_studio::settings::SettingsStore;
        #[allow(private_interfaces)]
        $vis type $name = $t;
        impl SettingsStore<$name> for $name {}
    };
    () => {};
}
