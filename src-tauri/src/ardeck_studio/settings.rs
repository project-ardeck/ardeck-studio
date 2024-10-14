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

pub mod format;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{atomic::AtomicBool, Mutex, OnceLock};
use std::{fs, io};
use tauri::utils::config;

use crate::service::dir::{self, Directories};

static WAS_CHANGED_SETTING: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum GetDeviceSettingError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

// TODO: generics
pub trait Settings<T: DeserializeOwned + Serialize> {
    fn config_file() -> &'static str;

    fn config_path() -> PathBuf {
        Directories::get_config_dir().join(Self::config_file())
    }

    fn get_config() -> Option<Result<T, serde_json::Error>> {
        let file = match File::open(Self::config_path()) {
            Ok(f) => f,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        return None;
                    }
                    _ => {
                        eprintln!("Error opening config file: {}", e);
                        return None;
                    }
                }
            }
        };
        let reader = BufReader::new(file);

        let json: Result<T, serde_json::Error> = from_reader(reader);

        Some(json)
    }

    fn save_config(data: T) {
        let mut file = match File::open(Self::config_path()) {
            Ok(f) => f,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        return;
                    }
                    _ => return, // TODO: match
                }
            }
        };
        let string = match serde_json::to_string(&data) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Serialization error: {}", e);
                return;
            }
        };
        file.write_all(string.as_bytes());
        // TODO: save function
    }
}
