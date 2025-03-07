/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 Project Ardeck

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

use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use uuid::Uuid;

use crate::{
    ardeck_studio::{
        action::action_map::ActionMap,
        settings::definitions::{ardeck::ArdeckProfileConfigItem, mapping_presets::MappingPreset},
        switch_info::SwitchType,
    },
    service::dir::Directories,
};

use super::{
    definitions::{ardeck::ArdeckProfileConfigJSON, mapping_presets::MappingPresetsJSON},
    SettingsStore,
};

// Mapping presets
#[tauri::command]
async fn get_mapping_list<R: Runtime>(
    app: tauri::AppHandle<R>,
    // mapping_presets_json: State<'_, Mutex<MappingPresetsJSON>>,
) -> Result<Vec<(String, String)>, String> {
    let mapping_presets = MappingPresetsJSON::new().load_force().await.unwrap();
    let list: Vec<(String, String)> = mapping_presets
        .iter()
        .map(|a| (a.uuid.clone(), a.preset_name.clone()))
        .collect();

    log::debug!("get_mapping_list: {:#?}", list);

    Ok(list)
}

// Mapping presets
#[tauri::command]
async fn get_mapping_preset<R: Runtime>(
    app: tauri::AppHandle<R>,
    // mapping_presets_json: State<'_, Mutex<MappingPresetsJSON>>,
    uuid: &str,
) -> Result<Option<MappingPreset>, String> {
    log::debug!("get_mapping_preset: {}", uuid);

    log::debug!("get_mapping_preset.uuid: {}", uuid);

    let mapping_presets = MappingPresetsJSON::new().load().await.unwrap();
    for a in mapping_presets.iter() {
        log::debug!("\tuuid: {}", a.uuid);

        if a.uuid == uuid.to_string() {
            log::debug!("\tfound.");
            return Ok(Some(a.clone()));
        }
    }
    log::debug!("\tnot found.");
    Ok(None)
}

// Mapping presets
#[tauri::command]
async fn save_mapping_preset<R: Runtime>(
    app: tauri::AppHandle<R>,
    // mapping_presets_json: State<'_, Mutex<MappingPresetsJSON>>,
    mut mapping_preset: MappingPreset,
) -> Result<MappingPreset, String> {
    let mut mapping_presets = MappingPresetsJSON::new().load().await.unwrap();
    log::debug!("save_mapping_preset: {:#?}", mapping_preset);

    log::debug!("save_mapping_preset.uuid: {}", mapping_preset.uuid);

    // すでに存在するかを確認する
    let index = mapping_presets
        .iter()
        .position(|p| p.uuid == mapping_preset.uuid);
    match index {
        // 存在したら、上書きする
        Some(i) => {
            mapping_presets[i] = mapping_preset.clone();

            log::debug!("save_mapping_preset.data_change");
            log::debug!("mapping_presets[after]: {:#?}", mapping_presets);

            mapping_presets.save().await;
        }
        // 存在しなければ、新規追加する
        None => {
            //TODO: add new mapping
            mapping_preset.uuid = Uuid::new_v4().to_string();

            mapping_presets.push(mapping_preset.clone());

            log::debug!("save_mapping_preset.new_data");
            log::debug!("mapping_presets[after]: {:#?}", mapping_presets);

            mapping_presets.save().await;
        }
    }

    Ok(mapping_preset)
}

// Ardeck Profile
type DeviceName = Option<String>;
type DeviceId = String;
type ProfileList = Vec<(DeviceId, DeviceName)>;
#[tauri::command]
async fn get_ardeck_profile_list<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<ProfileList, String> {
    let config = ArdeckProfileConfigJSON::new().load().await;
    let mut list: ProfileList = Vec::new();

    if config.is_none() {
        return Err("Failed to load ardeck profile config".into());
    }

    let config = config.unwrap();

    for item in config.iter() {
        list.push((item.device_id.clone(), item.device_name.clone()));
    }

    Ok(list)
}

#[tauri::command]
async fn get_ardeck_profile<R: Runtime>(
    app: tauri::AppHandle<R>,
    device_id: &str,
) -> Result<Option<ArdeckProfileConfigItem>, String> {
    let config = ArdeckProfileConfigJSON::new().load().await;
    if config.is_none() {
        return Err("Failed to load ardeck profile config".into());
    }

    let config = config.unwrap();

    for item in config.iter() {
        if item.device_id == device_id {
            return Ok(Some(item.clone()));
        }
    }

    Err("Failed to find ardeck profile config".into())
}

#[tauri::command]
async fn save_ardeck_profile<R: Runtime>(
    app: tauri::AppHandle<R>,
    profile: ArdeckProfileConfigItem,
) -> Result<ArdeckProfileConfigItem, String> {
    let mut config = ArdeckProfileConfigJSON::new().load().await.unwrap();

    let position = config.iter().position(|p| p.device_id == profile.device_id);

    match position {
        Some(i) => {
            config[i] = profile.clone();
        }
        None => {
            config.push(profile.clone());
        }
    }

    config.save().await;

    Ok(profile)
}

macro_rules! ext_config_file {
    () => {};
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("settings")
        .setup(|app| {
            // TODO: get_config_dir() log
            Directories::init(Directories::get_settings_dir().unwrap()).unwrap();

            Ok(())
        })
        .invoke_handler(generate_handler![
            get_mapping_list,
            get_mapping_preset,
            save_mapping_preset,
            get_ardeck_profile_list,
            get_ardeck_profile,
            save_ardeck_profile
        ])
        .build()
}
