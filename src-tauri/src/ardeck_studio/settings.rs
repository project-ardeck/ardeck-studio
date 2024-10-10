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
pub mod plugin;
pub mod mapping_presets;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::sync::{atomic::AtomicBool, Mutex, OnceLock};
use std::{fs, io};

static WAS_CHANGED_SETTING: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum GetDeviceSettingError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

pub trait Settings {
    fn config_path() -> &'static str;

    fn get_config() -> Option<Result<serde_json::Value, serde_json::Error>> {
        let file = match File::open(Self::config_path()) {
            Ok(f) => f,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        return None;
                    }
                    _ => return None // TODO: match
                }
            }
        };
        let reader = BufReader::new(file);

        let json: Result<serde_json::Value, serde_json::Error> = from_reader(reader);

        Some(json)
    }

    fn save_config() {
        // TODO: save function
    }
}
