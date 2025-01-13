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

use std::{collections::HashMap, io, path::PathBuf};

use cache::Cache;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;

use crate::service::file::Files;

pub mod definitions;
pub mod tauri;
pub mod cache;


#[derive(Debug)]
pub enum SettingsStoreError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

pub trait Settings {
    fn name(&self) -> &'static str;
    fn dir(&self) -> PathBuf;
}

// TODO: saveしたらフラグを立て、loadするときに確認し、フラグが立っていなければ前回のデータをそのまま返すようなキャッシュ機能を作成する
// static CACHE: Lazy<Mutex<Cache<(), ()>>> = Lazy::new(|| Mutex::new(Cache::new()));

pub trait SettingsStore<T>: Serialize + DeserializeOwned + Default + Clone + Send + Sync + Settings {
    fn file_path(&self) -> PathBuf {
        self.dir().join(format!("{}.json", self.name()))
    }

    fn load(&mut self) -> Self {


        let file = match Files::open(self.file_path()) {
            Ok(file) => file,
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        return Self::default();
                    },
                    _ => panic!("SettingsStore panic!: load.open"),
                }
            },
        };

        let reader = std::io::BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(setting) => {
                self.clone_from(&setting);
                self.clone()
            },
            Err(_) => panic!("SettingStore panic!: load.serialize"),
        }
    }

    fn save(&self) { // TODO: Result
        let file = match Files::create(self.file_path()) {
            Ok(file) => file,
            Err(_) => return,
        };
        let writer = std::io::BufWriter::new(file);
        match serde_json::to_writer_pretty(writer, self) {
            Ok(_) => (),
            Err(_) => return,
        };
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
