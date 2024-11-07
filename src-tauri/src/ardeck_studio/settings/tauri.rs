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

use std::vec;

use once_cell::sync::Lazy;
use tauri::{generate_handler, plugin::{Builder, TauriPlugin}, Manager, Runtime, State};

use super::definitions::{ardeck::ArdeckProfileConfigJSON, mapping_presets::MappingPresetsJSON, Setting, SettingEnum};

const SETTINGS: Lazy<Vec<SettingEnum>> = Lazy::new(|| vec![
    SettingEnum::ArdeckProfileConfig(ArdeckProfileConfigJSON::new()),
    SettingEnum::MappingPresets(MappingPresetsJSON::new()),
]);

// const SETTINGS: Lazy<Vec<Box<dyn Setting>>> = Lazy::new(|| vec![
//     Box::new(MappingPresetsJSON::new()),
//     Box::new(ArdeckProfileConfigJSON::new()),
// ]);

#[tauri::command]
fn get_setting_list<R: Runtime>(app: tauri::AppHandle<R>) -> Vec<&'static str> {
    SETTINGS.iter().map(|s| s.config_file()).collect()
}

#[tauri::command]
fn get_setting<R: Runtime>(app: tauri::AppHandle<R>, setting_path: &str) -> Option<SettingEnum> {
    SETTINGS
        .iter()
        .find(|setting| setting.config_file() == setting_path)
        .cloned()
}

#[tauri::command]
async fn save_setting<R: Runtime>(app: tauri::AppHandle<R>, setting:SettingEnum) -> Result<(), String> {
    match setting.save() {
        Ok(_) => Ok(()),
        Err(_) => Err("Error".to_string()), // TODO: Error handling
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("settings")
        .setup(|app| {
            // app.manage(file_name);

            Ok(())
        })
        .invoke_handler(generate_handler![get_setting, get_setting_list, save_setting])
        .build()
}
