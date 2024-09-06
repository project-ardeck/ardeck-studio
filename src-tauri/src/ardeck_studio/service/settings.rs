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


use std::sync::{atomic::AtomicBool, Mutex, OnceLock};
use std::fs;
use serde::{Deserialize, Serialize};

static WAS_CHANGED_SETTING: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSettingOptions {
    pub serial_number: String,

    pub device_name: Option<String>,
    pub baud_rate: Option<u32>, // default: 19200
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum GetDeviceSettingError {
    NotFound,
    // IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceSettings; // TODO: DeviceSettings -> Settings
impl DeviceSettings {
    // TODO: アプリで使うディレクトリを(存在しなければ)作成する関数
    pub fn init_dir() {}

    pub fn get_settings() -> Result<Vec<DeviceSettingOptions>, GetDeviceSettingError> {
        let settings_path = "settings/device_settings.json";
        let settings_str = match fs::read_to_string(settings_path) {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        match serde_json::from_str(&settings_str) {
            Ok(settings) => Ok(settings),
            Err(e) => Err(GetDeviceSettingError::SerdeError(e)),
        }
    }

    pub fn get_settings_device(serial_number: &str) -> Result<DeviceSettingOptions, GetDeviceSettingError> {
        let settings = Self::get_settings()?;
        for setting in settings {
            if setting.serial_number == serial_number {
                return Ok(setting);
            }
        }
        Err(GetDeviceSettingError::NotFound)
    }
    
    pub fn set_setting_studio(key: String, value: String) {} // ardeck studioの設定
    
    pub fn set_setting_ardeck() {}
}

pub struct SettingsStudio {
}

impl SettingsStudio {
    pub fn Theme(id: String) {}
}
