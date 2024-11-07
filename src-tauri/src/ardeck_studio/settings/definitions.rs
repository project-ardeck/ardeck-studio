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

pub mod ardeck;
pub mod ardeck_studio;
pub mod mapping_presets;
pub mod plugin;

use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
    path::PathBuf,
};

use ardeck::ArdeckProfileConfigJSON;
use chrono::format;
use derive_builder::Builder;
use mapping_presets::MappingPresetsJSON;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::from_reader;

use crate::service::dir::{self, Directories};

#[derive(Debug)]
pub enum SettingStoreError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

macro_rules! ext_config_file {
    () => {
        
    };
}

pub trait Setting: DeserializeOwned + Serialize + Send + Sync {
    // config_id: config file name
    fn config_id(&self) -> &'static str;

    // config file path
    // * [dir]/[config_id].json
    fn get_config_file_path(&self) -> PathBuf {
        Directories::get_config_dir().join(format!("{}.json", self.config_id()))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SettingEnum {
    MappingPresets(MappingPresetsJSON),
    ArdeckProfileConfig(ArdeckProfileConfigJSON),
}

impl SettingEnum {
    pub fn config_id(&self) -> &'static str {
        match self { // TODO: macro
            Self::MappingPresets(s) => s.config_id(),
            Self::ArdeckProfileConfig(s) => s.config_id()
        }
    }

    pub fn get_config_file_path(&self) -> PathBuf {
        match self {
            Self::MappingPresets(s) => s.get_config_file_path(),
            Self::ArdeckProfileConfig(s) => s.get_config_file_path(),
        }
    }

    pub fn load(&self) -> Result<Self, SettingStoreError> {
        let file = match File::open(self.get_config_file_path()) {
            Ok(it) => it,
            Err(err) => return Err(SettingStoreError::IoError(err)),
        };
        let reader = BufReader::new(file);
        let setting: Self = match serde_json::from_reader(reader) {
            Ok(it) => it,
            Err(err) => return Err(SettingStoreError::SerdeError(err)),
        };
        Ok(setting)
    }

    pub fn save(&self) -> Result<(), SettingStoreError> {
        let file = match File::create(self.get_config_file_path()) {
            Ok(it) => it,
            Err(err) => return Err(SettingStoreError::IoError(err)),
        };
        let writer = BufWriter::new(file);
        match serde_json::to_writer_pretty(writer, &self) {
            Ok(it) => it,
            Err(err) => return Err(SettingStoreError::SerdeError(err)),
        };
        Ok(())
    }
}


