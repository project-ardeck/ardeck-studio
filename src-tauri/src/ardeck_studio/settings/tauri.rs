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
    io::{BufReader, BufWriter},
    path::PathBuf,
    vec,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::{
    generate_handler, plugin::{Builder, TauriPlugin}, Manager, Runtime
};
use uuid::Uuid;

use crate::{ardeck_studio::{action::{map::ActionMap, SwitchType}, settings::definitions::mapping_presets::MappingPreset}, service::{dir::Directories, file::Files}};

use super::{
    definitions::{ardeck::ArdeckProfileConfigJSON, mapping_presets::MappingPresetsJSON},
    Settings, SettingsStore, SettingsStoreError,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SettingEnum {
    MappingPresets(MappingPresetsJSON),
    ArdeckProfileConfig(ArdeckProfileConfigJSON),
}

const SETTINGS: Lazy<Vec<SettingEnum>> = Lazy::new(|| {
    vec![
        SettingEnum::ArdeckProfileConfig(ArdeckProfileConfigJSON::new()),
        SettingEnum::MappingPresets(MappingPresetsJSON::new()),
    ]
});

macro_rules! ext_config_file {
    () => {
        
    };
}

impl SettingEnum {
    pub fn get_name(&self) -> &'static str {
        match self {
            // TODO: macro
            Self::MappingPresets(s) => s.name(),
            Self::ArdeckProfileConfig(s) => s.name(),
        }
    }

    pub fn get_file_path(&self) -> PathBuf {
        match self {
            Self::MappingPresets(s) => s.file_path(),
            Self::ArdeckProfileConfig(s) => s.file_path(),
        }
    }

    pub fn init(&self) -> Result<(), SettingsStoreError> {
        // TODO: これは仮
        self.save()
    }

    pub fn load(&self) -> Self {
        match self {
            SettingEnum::ArdeckProfileConfig(setting) => {
                let setting = setting.load();
                return SettingEnum::ArdeckProfileConfig(setting);
            },
            SettingEnum::MappingPresets(setting) => {
                let setting = setting.load();
                return SettingEnum::MappingPresets(setting);
            }
        }
    }

    pub fn save(&self) -> Result<(), SettingsStoreError> {
        
        match self {
            SettingEnum::ArdeckProfileConfig(setting) => Ok(setting.save()), // TODO: Error handling
            SettingEnum::MappingPresets(setting) => Ok(setting.save()),
        }
    }
}

#[tauri::command]
fn get_setting_list<R: Runtime>(_app: tauri::AppHandle<R>) -> Vec<&'static str> {
    SETTINGS.iter().map(|s| s.get_name()).collect()
}

#[tauri::command]
fn get_setting<R: Runtime>(_app: tauri::AppHandle<R>, config_id: &str) -> SettingEnum {
    let setting = SETTINGS
        .iter()
        .find(|setting| setting.get_name() == config_id)
        .cloned();

    setting.unwrap().load()
}

// Mapping presets
// #[tauri::command]
// async fn get_mapping_uuid_list<R: Runtime>(app: tauri::AppHandle<R>) -> Result<Vec<String>, String> {
//     Ok(())
// }

#[tauri::command]
async fn save_setting<R: Runtime>(
    _app: tauri::AppHandle<R>,
    setting: SettingEnum,
) -> Result<(), String> {
    match setting.save() {
        Ok(_) => Ok(()),
        Err(_) => Err("Error".to_string()), // TODO: Error handling
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("settings")
        .setup(|app| {
            Directories::init(Directories::get_config_dir()).unwrap();

            for setting in SETTINGS.iter() {
                println!("{:?}", setting);

                let setting = setting.load();

                // setting.save().unwrap();

                match setting.clone() {
                    SettingEnum::ArdeckProfileConfig(setting) => {
                        setting.save();
                    },
                    SettingEnum::MappingPresets(mut setting) => {
                        let sample_data = MappingPreset {
                            uuid: Uuid::new_v4().to_string(),
                            preset_name: Some("sample".to_string()),

                            mapping: vec![
                                ActionMap {
                                    switch_type: SwitchType::Digital,
                                    switch_id: 1,
                                    plugin_id: "sample_plugin".to_string(),
                                    action_id: "sample_action_1".to_string()
                                },
                                ActionMap {
                                    switch_type: SwitchType::Digital,
                                    switch_id: 2,
                                    plugin_id: "sample_plugin".to_string(),
                                    action_id: "sample_action_2".to_string()
                                },
                                ActionMap {
                                    switch_type: SwitchType::Analog,
                                    switch_id: 3,
                                    plugin_id: "sample_plugin".to_string(),
                                    action_id: "sample_action_3".to_string()
                                },
                            ],
                        };

                        while setting.len() < 1 {
                            setting.push(sample_data.clone());
                        }

                        setting.save();
                    }
                }
            }

            Ok(())
        })
        .invoke_handler(generate_handler![
            get_setting,
            get_setting_list,
            save_setting
        ])
        .build()
}
