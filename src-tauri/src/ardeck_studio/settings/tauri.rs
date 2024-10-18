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
use tauri::{plugin::{Builder, TauriPlugin}, Manager, Runtime, State};

use super::definitions::{ardeck::ArdeckProfileConfigJSON, mapping_presets::MappingPresetsJSON, Settings};

const SETTINGS: Lazy<Vec<Settings>> = Lazy::new(|| vec![
    ArdeckProfileConfigJSON
]);

#[tauri::command]
fn get_setting_list(list: State<Vec<&str>>) -> Vec<&'static str> {
}

pub async fn init<R: Runtime>() -> TauriPlugin<R> {
    let file_name = vec![
        MappingPresetsJSON::config_file(),
        ArdeckProfileConfigJSON::config_file(),
    ];

    Builder::new("settings")
        .setup(|app| {
            app.manage(file_name);

            Ok(())
        })
        .build()
}
