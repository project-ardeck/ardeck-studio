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
        action::action_map::ActionMap, settings::definitions::mapping_presets::MappingPreset,
        switch_info::SwitchType,
    },
    service::dir::Directories,
};

use super::{definitions::mapping_presets::MappingPresetsJSON, SettingsStore};

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

    // println!("get_mapping_list\n\tmapping_presets_json: {:#?}", mapping_presets_json.lock().unwrap());

    Ok(list)
}

#[tauri::command]
async fn get_mapping_preset<R: Runtime>(
    app: tauri::AppHandle<R>,
    // mapping_presets_json: State<'_, Mutex<MappingPresetsJSON>>,
    uuid: &str,
) -> Result<Option<MappingPreset>, String> {
    println!("get_mapping_preset: {}", uuid);

    // println!("get_mapping_preset\n\tmapping_presets_json: {:#?}", mapping_presets_json.lock().unwrap());

    let mapping_presets = MappingPresetsJSON::new().load().await.unwrap();
    for a in mapping_presets.iter() {
        println!("\tuuid: {}", a.uuid);
        if a.uuid == uuid.to_string() {
            println!("\tfound.");
            return Ok(Some(a.clone()));
        }
    }
    println!("\tnot found.");
    Ok(None)
}

#[tauri::command]
async fn save_mapping_preset<R: Runtime>(
    app: tauri::AppHandle<R>,
    // mapping_presets_json: State<'_, Mutex<MappingPresetsJSON>>,
    mut mapping_preset: MappingPreset,
) -> Result<MappingPreset, String> {
    let mut mapping_presets = MappingPresetsJSON::new().load().await.unwrap();
    println!("mapping_presets[before]: {:#?}", mapping_presets);

    // println!("save_mapping_preset\n\tmapping_presets_json: {:#?}\n\tmapping_preset: {:#?}", mapping_presets_json.lock().unwrap(), mapping_preset);
    println!("save_mapping_preset.uuid: {}", mapping_preset.uuid);

    // すでに存在するかを確認する
    let index = mapping_presets
        .iter()
        .position(|p| p.uuid == mapping_preset.uuid);
    match index {
        // 存在したら、上書きする
        Some(i) => {
            mapping_presets[i] = mapping_preset.clone();

            // println!("save_mapping_preset.data_change\n\tmapping_presets_json: {:#?}", mapping_presets_json.lock().unwrap());
            println!("save_mapping_preset.data_change",);
            println!("mapping_presets[after]: {:#?}", mapping_presets);

            mapping_presets.save().await;
        }
        // 存在しなければ、新規追加する
        None => {
            //TODO: add new mapping
            mapping_preset.uuid = Uuid::new_v4().to_string();

            mapping_presets.push(mapping_preset.clone());

            println!(
                "save_mapping_preset.new_data",
                // mapping_presets_json.lock().unwrap()
            );
            println!("mapping_presets[after]: {:#?}", mapping_presets);

            mapping_presets.save().await;
        }
    }

    // println!("mapping_presets: {:#?}", mapping_presets);

    // mapping_presets_json.lock().unwrap()./*.unwrap()*/;
    // setting.save(&mapping_preset).await;

    Ok(mapping_preset)
}

macro_rules! ext_config_file {
    () => {};
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("settings")
        .setup(|app| {
            // TODO: get_config_dir() log
            Directories::init(Directories::get_settings_dir().unwrap()).unwrap();
            // app.manage(Mutex::new(MappingPresetsJSON::new()));

            let sample_data = MappingPreset {
                uuid: Uuid::new_v4().to_string(),
                preset_name: "sample".to_string(),

                mapping: vec![
                    ActionMap {
                        switch_type: SwitchType::Digital,
                        switch_id: 1,
                        plugin_id: "sample_plugin".to_string(),
                        action_id: "sample_action_1".to_string(),
                    },
                    ActionMap {
                        switch_type: SwitchType::Digital,
                        switch_id: 2,
                        plugin_id: "sample_plugin".to_string(),
                        action_id: "sample_action_2".to_string(),
                    },
                    ActionMap {
                        switch_type: SwitchType::Analog,
                        switch_id: 3,
                        plugin_id: "sample_plugin".to_string(),
                        action_id: "sample_action_3".to_string(),
                    },
                ],
            };

            // let mut mapping = MAPPING_PRESETS.load();
            // // TODO: グローバル変数やめる
            // if mapping.is_empty() {
            //     mapping.push(sample_data);
            // }

            // mapping.save();

            Ok(())
        })
        .invoke_handler(generate_handler![
            get_mapping_list,
            get_mapping_preset,
            save_mapping_preset
        ])
        .build()
}
