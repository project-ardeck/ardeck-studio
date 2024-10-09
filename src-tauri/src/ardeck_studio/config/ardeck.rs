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

use std::fs;

use serde::{Deserialize, Serialize};

use super::GetDeviceSettingError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArdeckProfileConfigOption {
    pub serial_number: String,

    pub device_name: Option<String>,
    pub baud_rate: Option<u32>, // default: 19200
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArdeckProfileConfig; // TODO: DeviceSettings -> Settings
impl ArdeckProfileConfig {
    pub fn get_config() -> Result<Vec<ArdeckProfileConfigOption>, GetDeviceSettingError> {
        let settings_path = "config/device_settings.json";
        let settings_str = match fs::read_to_string(settings_path) {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        match serde_json::from_str(&settings_str) {
            Ok(settings) => Ok(settings),
            Err(e) => Err(GetDeviceSettingError::SerdeError(e)),
        }
    }

    pub fn get_settings_device(serial_number: &str) -> Result<ArdeckProfileConfigOption, GetDeviceSettingError> {
        let settings = Self::get_config()?;
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
