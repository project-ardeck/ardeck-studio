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

use std::{
    fs::File,
    io::{self, BufReader, Write},
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_reader, Value};

use crate::ardeck_studio::settings::definitions::Setting;

// TODO: remove service

// #[derive(Debug)]
// pub enum SettingStoreError {
//     IoError(io::Error),
//     SerdeError(serde_json::Error),
// }

// pub fn load(s: &(impl Setting + DeserializeOwned)) -> Result<Value, SettingStoreError> {
//     let file = match File::open(s.get_config_file_path()) {
//         Ok(f) => f,
//         Err(e) => {
//             return Err(SettingStoreError::IoError(e));
//         }
//     };
//     let reader = BufReader::new(file);
//     let json: Value = match from_reader(reader) {
//         Ok(j) => j,
//         Err(e) => {
//             return Err(SettingStoreError::SerdeError(e));
//         }
//     };
//     Ok(json)
// }

// pub fn save(s: &(impl Setting + Serialize)) -> Result<(), SettingStoreError> {
//     let mut file = match File::open(&s.get_config_file_path()) {
//         Ok(f) => f,
//         Err(e) => {
//             eprintln!("File open error: {}", e);
//             return Err(SettingStoreError::IoError(e));
//         }
//     };
//     let string = match serde_json::to_string_pretty(&s) {
//         Ok(s) => s,
//         Err(e) => {
//             eprintln!("Serialization error: {}", e);
//             return Err(SettingStoreError::SerdeError(e));
//         }
//     };
//     if let Err(e) = file.write_all(string.as_bytes()) {
//         eprintln!("File write error: {}", e);
//         return Err(SettingStoreError::IoError(e));
//     }
//     // TODO: save function

//     Ok(())
// }
