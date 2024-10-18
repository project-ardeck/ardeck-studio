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

use std::{fs::File, io::{self, BufReader, Write}};

use serde_json::from_reader;

use crate::ardeck_studio::settings::definitions::Settings;


#[derive(Debug)]
pub enum FileGetError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}


struct Store;
impl Store {
    pub fn get<S: Settings>(s: S) -> Option<Result<S, serde_json::Error>> {
        let file = match File::open(S::config_file()) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    return None;
                }
                _ => {
                    eprintln!("Error opening config file: {}", e);
                    return None;
                }
            },
        };
        let reader = BufReader::new(file);

        let json: Result<S, serde_json::Error> = from_reader(reader);

        Some(json)
    }

    pub fn save<S: Settings>(s: S) -> Result<(), FileGetError> {
        let mut file = match File::open(S::get_config_file_path()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("File open error: {}", e);
                return Err(FileGetError::IoError(e));
            }
        };
        let string = match serde_json::to_string(&s) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Serialization error: {}", e);
                return Err(FileGetError::SerdeError(e));
            }
        };
        if let Err(e) = file.write_all(string.as_bytes()) {
            eprintln!("File write error: {}", e);
            return Err(FileGetError::IoError(e));
        }
        // TODO: save function

        Ok(())
    }
}
